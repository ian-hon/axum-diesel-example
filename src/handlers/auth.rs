use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
use uuid::Uuid;

use crate::error::{AppError, JsonRejection};
use crate::middleware::auth::JwtAccessTokenClaims;
use crate::models::User;
use crate::state::{
    AccessTokenAudience, AccessTokenClientId, AccessTokenExpiration, AccessTokenIssuer,
    JwsSigningSecret,
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
    State(users): State<Arc<Mutex<HashMap<Uuid, User>>>>,
    State(access_token_issuer): State<AccessTokenIssuer>,
    State(access_token_expiration): State<AccessTokenExpiration>,
    State(access_token_audience): State<AccessTokenAudience>,
    State(access_token_client_id): State<AccessTokenClientId>,
    State(jws_signing_secret): State<JwsSigningSecret>,
    WithRejection(Json(payload), _): WithRejection<Json<PostLoginPayload>, JsonRejection>,
) -> Result<Json<PostLoginResponse>> {
    let now = chrono::Utc::now();

    // Check if the user exists.
    //
    // # Security
    //
    // Never reveal whether the user exists or not.
    //
    // See <https://owasp.org/www-project-web-security-testing-guide/stable/4-Web_Application_Security_Testing/03-Identity_Management_Testing/04-Testing_for_Account_Enumeration_and_Guessable_User_Account>
    if let Some(user) = users
        .lock()
        .unwrap()
        .values()
        .find(|&user| user.username == payload.username)
    {
        // Check if the password matches.
        password_auth::verify_password(
            payload.password.expose_secret(),
            user.password_hash.expose_secret(),
        )
        .map_err(|_err| {
            (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "title": "InvalidUsernameOrPassword",
                })),
            )
        })?;

        let access_token_max_age = access_token_expiration
            .0
            .to_duration(SpanRelativeTo::days_are_24_hours())
            .map_err(anyhow::Error::from)
            .and_then(|duration| {
                std::time::Duration::try_from(duration).map_err(anyhow::Error::from)
            })
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

        return Ok(Json(PostLoginResponse {
            id: user.id,
            access_token,
        }));
    }

    Err((
        StatusCode::FORBIDDEN,
        Json(json!({
            "title": "InvalidUsernameOrPassword",
        })),
    ))?
}

pub async fn post_signup(
    State(users): State<Arc<Mutex<HashMap<Uuid, User>>>>,
    WithRejection(Json(payload), _): WithRejection<Json<PostSignupPayload>, JsonRejection>,
) -> Result<Json<PostSignUpResponse>> {
    // Check if the user exists.
    if users
        .lock()
        .unwrap()
        .values()
        .any(|user| user.username == payload.username)
    {
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

    let user_id = Uuid::now_v7();
    let password_hash = password_auth::generate_hash(payload.password.expose_secret()).into();
    let balance = BigDecimal::from(rand::random_range(..=u16::MAX));

    let user = User {
        id: user_id,
        username: payload.username,
        password_hash,
        balance,
    };
    users.lock().unwrap().insert(user_id, user);

    Ok(Json(PostSignUpResponse { id: user_id }))
}
