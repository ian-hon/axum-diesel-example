use axum::Router;
use axum::routing::post;
use axum_extra::vpath;

use crate::handlers::transactions::post_fetch_all;
use crate::handlers::transactions::post_transfer;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(vpath!("/transactions/transfer"), post(post_transfer))
        .route(vpath!("/transactions/fetch_all"), post(post_fetch_all))
}
