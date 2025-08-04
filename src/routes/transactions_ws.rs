use axum::Router;
use axum::routing::post;
use axum_extra::vpath;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route(
        vpath!("/transactions/ws"),
        post(async || "not implemented yet"),
    )
}
