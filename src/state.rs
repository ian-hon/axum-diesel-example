use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::FromRef;
use jiff::Span;
use secrecy::SecretSlice;
use url::Url;
use uuid::Uuid;

use crate::models::{Transaction, User};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<Uuid, User>>>,
    pub transactions: Arc<Mutex<HashMap<Uuid, Transaction>>>,
}

#[derive(Clone, FromRef)]
pub struct AuthState {
    pub jws_signing_secret: JwsSigningSecret,
    pub access_token_issuer: AccessTokenIssuer,
    pub access_token_expiration: AccessTokenExpiration,
    pub access_token_audience: AccessTokenAudience,
    pub access_token_client_id: AccessTokenClientId,
    pub users: Arc<Mutex<HashMap<Uuid, User>>>,
}

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
