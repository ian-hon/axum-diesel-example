use axum::Router;
use axum::routing::get;
use axum_extra::vpath;

use crate::handlers::user::{get_transactions, get_user};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(vpath!("/{user_id}"), get(get_user))
        .route(vpath!("/{user_id}/transactions"), get(get_transactions))
}
