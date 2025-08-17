use axum::extract::FromRef;
use diesel::SqliteConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use jiff::Span;
use secrecy::SecretSlice;
use url::Url;
use uuid::Uuid;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_connection_pool: DbConnectionPool,
}

#[derive(Clone, FromRef)]
pub struct AuthState {
    pub db_connection_pool: DbConnectionPool,
    pub jws_signing_secret: JwsSigningSecret,
    pub access_token_issuer: AccessTokenIssuer,
    pub access_token_expiration: AccessTokenExpiration,
    pub access_token_audience: AccessTokenAudience,
    pub access_token_client_id: AccessTokenClientId,
}

pub type DbConnectionPool = Pool<SyncConnectionWrapper<SqliteConnection>>;

#[derive(Clone)]
pub struct JwsSigningSecret(pub SecretSlice<u8>);

#[derive(Clone)]
pub struct AccessTokenIssuer(pub Url);

#[derive(Copy, Clone)]
pub struct AccessTokenExpiration(pub Span);

#[derive(Clone)]
pub struct AccessTokenAudience(pub Url);

#[derive(Copy, Clone)]
pub struct AccessTokenClientId(pub Uuid);
