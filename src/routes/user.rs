use axum::Router;
use axum::routing::post;
use axum_extra::vpath;

use crate::handlers::user::post_fetch_balance;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route(vpath!("/user/balance"), post(post_fetch_balance))
}
