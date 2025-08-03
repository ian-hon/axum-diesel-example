use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::FromRef;
use tokio::sync::Mutex;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub user_data: Arc<Mutex<HashMap<String, String>>>,
}
