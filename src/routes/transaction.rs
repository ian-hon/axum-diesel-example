use axum::Router;
use axum::routing::post;
use axum_extra::vpath;

use crate::handlers::transaction::post_transaction;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route(vpath!("/"), post(post_transaction))
}
