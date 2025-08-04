use axum::Router;
use axum::routing::post;
use axum_extra::vpath;

use crate::handlers::auth::{post_login, post_signup};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(vpath!("/auth/login"), post(post_login))
        .route(vpath!("/auth/signup"), post(post_signup))
}
