use anyhow::Context as _;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Result;
use axum_extra::extract::WithRejection;
use bigdecimal::BigDecimal;
use biscuit::SingleOrMultiple::Single;
use biscuit::jwa::SignatureAlgorithm;
use biscuit::{ClaimsSet, JWT, RegisteredClaims, jws};
use chrono::TimeDelta;
use jiff::SpanRelativeTo;
use secrecy::{ExposeSecret as _, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::error::{AppError, JsonRejection};
use crate::middleware::auth::JwtAccessTokenClaims;
use crate::models::User;
use crate::models::user::NewUser;
use crate::state::{
    AccessTokenAudience, AccessTokenClientId, AccessTokenExpiration, AccessTokenIssuer,
    DbConnectionPool, JwsSigningSecret,
};

#[derive(Debug, Deserialize)]
pub struct PostLoginPayload {
    username: String,
    password: SecretString,
}

#[derive(Serialize)]
pub struct PostLoginResponse {
    id: Uuid,
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct PostSignupPayload {
    username: String,
    password: SecretString,
}

#[derive(Serialize)]
pub struct PostSignUpResponse {
    id: Uuid,
}

pub async fn post_login(
    State(pool): State<DbConnectionPool>,
    State(access_token_issuer): State<AccessTokenIssuer>,
    State(access_token_expiration): State<AccessTokenExpiration>,
    State(access_token_audience): State<AccessTokenAudience>,
    State(access_token_client_id): State<AccessTokenClientId>,
    State(jws_signing_secret): State<JwsSigningSecret>,
    WithRejection(Json(payload), _): WithRejection<Json<PostLoginPayload>, JsonRejection>,
) -> Result<Json<PostLoginResponse>> {
    let mut conn = pool
        .get()
        .await
        .context("failed to get database connection")
        .map_err(AppError::from)?;

    // Check if the user exists.
    //
    // # Security
    //
    // Never reveal whether the user exists or not.
    //
    // See <https://owasp.org/www-project-web-security-testing-guide/stable/4-Web_Application_Security_Testing/03-Identity_Management_Testing/04-Testing_for_Account_Enumeration_and_Guessable_User_Account>
    let user = {
        use diesel::prelude::*;
        #[allow(
            clippy::unused_trait_names,
            reason = "error[E0034]: multiple applicable items in scope"
        )]
        use diesel_async::RunQueryDsl;

        use crate::schema::users;

        let user: User = match users::table
            .filter(users::username.eq(&payload.username))
            .select(User::as_select())
            .first(&mut conn)
            .await
        {
            Ok(user) => user,
            Err(diesel::NotFound) => {
                debug!(payload.username, "could not find user");

                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "title": "InvalidUsernameOrPassword",
                    })),
                ))?;
            },
            Err(err) => {
                return Err(err)
                    .context("failed to query users")
                    .map_err(AppError::from)?;
            },
        };
        user
    };

    // Check if the password matches.
    password_auth::verify_password(
        payload.password.expose_secret(),
        user.password_hash.expose_secret(),
    )
    .map_err(|_err| {
        debug!(payload.username, "wrong password");

        (
            StatusCode::FORBIDDEN,
            Json(json!({
                "title": "InvalidUsernameOrPassword",
            })),
        )
    })?;

    let now = chrono::Utc::now();

    let access_token_max_age = access_token_expiration
        .0
        .to_duration(SpanRelativeTo::days_are_24_hours())
        .map_err(anyhow::Error::from)
        .and_then(|duration| std::time::Duration::try_from(duration).map_err(anyhow::Error::from))
        .and_then(|duration| TimeDelta::from_std(duration).map_err(anyhow::Error::from))
        .expect("converting `access_token_expiration` should not fail");

    let access_token = JWT::<JwtAccessTokenClaims, biscuit::Empty>::new_decoded(
        jws::RegisteredHeader {
            algorithm: SignatureAlgorithm::HS256,
            media_type: Some("at+jwt".to_owned()),
            ..Default::default()
        }
        .into(),
        ClaimsSet {
            registered: RegisteredClaims {
                issuer: Some(access_token_issuer.0.as_str().to_owned()),
                expiry: Some(now.checked_add_signed(access_token_max_age).unwrap().into()),
                audience: Some(Single(access_token_audience.0.as_str().to_owned())),
                subject: Some(user.id.to_string()),
                issued_at: Some(now.into()),
                id: Some(Uuid::new_v4().to_string()),
                ..Default::default()
            },
            private: JwtAccessTokenClaims {
                client_id: access_token_client_id.0.to_string(),
            },
        },
    );
    let access_token = access_token
        .encode(&jws::Secret::Bytes(
            jws_signing_secret.0.expose_secret().to_vec(),
        ))
        .context("failed to encode and sign access token")
        .map_err(AppError::from)?;
    let access_token = access_token
        .encoded()
        .expect("`access_token` should be already encoded")
        .to_string();

    Ok(Json(PostLoginResponse {
        id: user.id,
        access_token,
    }))
}

pub async fn post_signup(
    State(pool): State<DbConnectionPool>,
    WithRejection(Json(payload), _): WithRejection<Json<PostSignupPayload>, JsonRejection>,
) -> Result<Json<PostSignUpResponse>> {
    use diesel::prelude::*;
    #[allow(
        clippy::unused_trait_names,
        reason = "error[E0034]: multiple applicable items in scope"
    )]
    use diesel_async::RunQueryDsl;

    use crate::schema::users;

    let mut conn = pool
        .get()
        .await
        .context("failed to get database connection")
        .map_err(AppError::from)?;

    // Check if the user exists.
    let existing_user: Option<User> = users::table
        .filter(users::username.eq(&payload.username))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .optional()
        .context("failed to query users")
        .map_err(AppError::from)?;
    if let Some(existing_user) = existing_user {
        debug!(payload.username, %existing_user.id, "user already exists");

        // Username already exists.
        //
        // # Security
        //
        // Oops... We don't have any way to protect against account enumeration in this
        // example. In production, you could potentially use email verification to avoid
        // revealing whether a user exists or not.
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "title": "UsernameTaken",
            })),
        ))?;
    }

    let password_hash = password_auth::generate_hash(payload.password.expose_secret()).into();
    let balance = BigDecimal::from(rand::random_range(..=u16::MAX));

    let new_user = NewUser {
        id: Uuid::now_v7(),
        username: payload.username,
        password_hash,
        balance,
    };

    let created_user: User = diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .context("failed to insert user")
        .map_err(AppError::from)?;

    Ok(Json(PostSignUpResponse {
        id: created_user.id,
    }))
}
