use std::path::PathBuf;
use std::{env, io};

use anyhow::{Context as _, Result, ensure};
use axum::error_handling::HandleErrorLayer;
use axum::http::request::Parts as RequestParts;
use axum::http::{HeaderValue, StatusCode, header};
use axum::routing::get;
use axum::{BoxError, Router, middleware};
use axum_diesel_example::jwt::HS256_SECRET_KEY_LEN;
use axum_diesel_example::middleware::auth::authenticate_with_jwt_access_token;
use axum_diesel_example::models::user::NewUser;
use axum_diesel_example::routes;
use axum_diesel_example::state::{
    AccessTokenAudience, AccessTokenClientId, AccessTokenExpiration, AccessTokenIssuer, AppState,
    AuthState, DbConnectionPool, JwsSigningSecret,
};
use axum_extra::vpath;
use base64ct::{Base64, Encoding as _};
use bigdecimal::BigDecimal;
use diesel::{ConnectionError, ConnectionResult, SqliteConnection};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::{AsyncConnection as _, SimpleAsyncConnection as _};
use futures_lite::FutureExt as _;
use secrecy::SecretSlice;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use uuid::Uuid;

const SERVICE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
const DB_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<()> {
    // Load some env vars from the `.env` file. Do not use this in production,
    // especially not for secrets.
    dotenvy::dotenv()?;

    init_tracing_subscriber();

    let db_url = env::var("DATABASE_URL").context("`DATABASE_URL` env var should be set")?;
    let db_connection_pool = {
        let mut config = ManagerConfig::default();
        config.custom_setup = Box::new(|db_url| establish_connection(db_url).boxed());
        let manager: AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>> =
            AsyncDieselConnectionManager::<_>::new_with_config(db_url, config);
        Pool::builder(manager)
            .create_timeout(Some(DB_CONNECT_TIMEOUT))
            .runtime(deadpool::Runtime::Tokio1)
            .build()?
    };

    create_user_fixtures(db_connection_pool.clone()).await?;

    let state = AppState {
        db_connection_pool: db_connection_pool.clone(),
    };

    let auth_state = AuthState {
        db_connection_pool,
        jws_signing_secret: JwsSigningSecret(SecretSlice::from({
            let secret = Base64::decode_vec(
                &env::var("JWS_SIGNING_HMAC_SECRET_KEY")
                    .context("`JWS_SIGNING_HMAC_SECRET_KEY` env var should be set")?,
            )
            .context("JWS signing HMAC secret key contains invalid Base64")?;
            ensure!(
                secret.len() == HS256_SECRET_KEY_LEN,
                "JWS signing HMAC secret key must be {HS256_SECRET_KEY_LEN} bytes"
            );
            secret
        })),
        access_token_issuer: AccessTokenIssuer(
            env::var("ACCESS_TOKEN_ISSUER")
                .context("`ACCESS_TOKEN_ISSUER` env var should be set")?
                .parse()
                .context("`ACCESS_TOKEN_ISSUER` env var should be a valid URL")?,
        ),
        access_token_expiration: AccessTokenExpiration(
            env::var("ACCESS_TOKEN_EXPIRATION")
                .context("`ACCESS_TOKEN_EXPIRATION` env var should be set")?
                .parse()
                .context("`ACCESS_TOKEN_EXPIRATION` env var should be a valid duration")?,
        ),
        access_token_audience: AccessTokenAudience(
            env::var("ACCESS_TOKEN_AUDIENCE")
                .context("`ACCESS_TOKEN_AUDIENCE` env var should be set")?
                .parse()
                .context("`ACCESS_TOKEN_AUDIENCE` env var should be a valid URL")?,
        ),
        access_token_client_id: AccessTokenClientId(
            env::var("ACCESS_TOKEN_CLIENT_ID")
                .context("`ACCESS_TOKEN_CLIENT_ID` env var should be set")?
                .parse()
                .context("`ACCESS_TOKEN_CLIENT_ID` env var should be a valid UUID")?,
        ),
    };

    // Serve the frontend as static files. In production you'd not want to serve
    // this from your API, but deployed separately, perhaps using a static file
    // serving service.
    let frontend_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend");
    let static_files_service = ServeDir::new(frontend_dir).append_index_html_on_directories(true);

    // Create the app router.
    let app = Router::new()
        .nest(vpath!("/users"), routes::user::routes())
        .nest(vpath!("/transactions"), routes::transaction::routes())
        .with_state(state)
        .layer(
            // Require authentication for any routes added before this middleware.
            //
            // See <https://docs.rs/axum/latest/axum/middleware/index.html#ordering>
            ServiceBuilder::new().layer(middleware::from_fn_with_state(
                auth_state.clone(),
                authenticate_with_jwt_access_token,
            )),
        )
        .route(vpath!("/hello"), get(async || "hello world!"))
        .nest(
            vpath!("/auth"),
            routes::auth::routes().with_state(auth_state),
        )
        .layer(
            // Use a service builder to compose multiple middleware layers. They are run from top
            // to bottom.
            //
            // See <https://docs.rs/axum/latest/axum/middleware/index.html#applying-multiple-middleware>
            // See <https://docs.rs/axum/latest/axum/middleware/index.html#ordering>
            ServiceBuilder::new()
                .layer(
                    // Add tracing for HTTP request-response lifecycle.
                    TraceLayer::new_for_http(),
                )
                .layer(
                    // Handle uncaught errors (from middleware) that have bubbled up to the top
                    // level.
                    //
                    // axum handlers do not bubble errors this way, as the error type is enforced
                    // to be `Infallible`.
                    HandleErrorLayer::new(|err: BoxError| async move {
                        if err.is::<tower::timeout::error::Elapsed>() {
                            // Convert a timeout error into a HTTP error response.
                            Ok(StatusCode::SERVICE_UNAVAILABLE)
                        } else {
                            error!(?err, "unhandled server error");
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "server error"))
                        }
                    }),
                )
                .timeout(
                    // Trigger a timeout error if we don't have a response after a certain period.
                    //
                    // This is converted into a HTTP error response in `HandleErrorLayer` above.
                    SERVICE_TIMEOUT,
                )
                .layer(
                    // Add CORS headers.
                    //
                    // Note that we allow any origin for convenience here. You probably don't want
                    // to do this in production.
                    CorsLayer::new()
                        .allow_origin(AllowOrigin::predicate(
                            |_origin: &HeaderValue, _request_parts: &RequestParts| true,
                        ))
                        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                        .allow_credentials(true),
                ),
        )
        .fallback_service(static_files_service);

    // Serve the app on port 8000.
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind to {addr}"))?;
    info!(addr = %listener.local_addr().unwrap(), "starting service");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                #[cfg(not(debug_assertions))]
                {
                    "info".into()
                }
                #[cfg(debug_assertions)]
                {
                    // Axum logs rejections from built-in extractors with the `axum::rejection`
                    // target, at `TRACE` level.
                    format!(
                        "{crate}=debug,tower_http=debug,axum::rejection=trace",
                        crate = env!("CARGO_CRATE_NAME"),
                    )
                    .into()
                }
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(io::stderr))
        .init();
}

async fn establish_connection(
    db_url: &str,
) -> ConnectionResult<SyncConnectionWrapper<SqliteConnection>> {
    let mut conn = SyncConnectionWrapper::<SqliteConnection>::establish(db_url).await?;

    // See <https://fractaledmind.github.io/2023/09/07/enhancing-rails-sqlite-fine-tuning/>

    // Sleep if the database is busy, this corresponds to up to 2 seconds sleeping
    // time.
    conn.batch_execute("PRAGMA busy_timeout = 2000;")
        .await
        .map_err(ConnectionError::CouldntSetupConfiguration)?;
    // Better write-concurrency.
    conn.batch_execute("PRAGMA journal_mode = WAL;")
        .await
        .map_err(ConnectionError::CouldntSetupConfiguration)?;
    // Fsync only in critical moments.
    conn.batch_execute("PRAGMA synchronous = NORMAL;")
        .await
        .map_err(ConnectionError::CouldntSetupConfiguration)?;
    // Write WAL changes back every 1000 pages, for a 1MB WAL file on average.
    // May affect readers if number is increased.
    conn.batch_execute("PRAGMA wal_autocheckpoint = 1000;")
        .await
        .map_err(ConnectionError::CouldntSetupConfiguration)?;
    // Free some space by truncating possibly massive WAL files from the last run.
    conn.batch_execute("PRAGMA wal_checkpoint(TRUNCATE);")
        .await
        .map_err(ConnectionError::CouldntSetupConfiguration)?;

    Ok(conn)
}

async fn create_user_fixtures(pool: DbConnectionPool) -> Result<()> {
    use axum_diesel_example::schema::users;
    #[allow(
        clippy::unused_trait_names,
        reason = "error[E0034]: multiple applicable items in scope"
    )]
    use diesel_async::RunQueryDsl;

    let mut conn = pool
        .get()
        .await
        .context("failed to get database connection")?;

    let new_users = vec![
        NewUser {
            id: Uuid::now_v7(),
            username: "john_doe".to_owned(),
            password_hash: password_auth::generate_hash("abc123").into(),
            balance: BigDecimal::from(12_345),
        },
        NewUser {
            id: Uuid::now_v7(),
            username: "mary_jane".to_owned(),
            password_hash: password_auth::generate_hash("password").into(),
            balance: BigDecimal::from(45_678),
        },
    ];

    // Insert these users if they don't already exist, otherwise do nothing.
    for new_user in new_users {
        diesel::insert_into(users::table)
            .values(new_user)
            .on_conflict(users::username)
            .do_nothing()
            .execute(&mut conn)
            .await
            .context("failed to insert user")?;
    }

    Ok(())
}
