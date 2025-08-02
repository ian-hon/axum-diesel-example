use std::collections::HashMap;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::routing::get;
use axum_diesel_example::routes;
use axum_diesel_example::state::AppState;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

pub async fn debug_command(State(s): State<AppState>) -> String {
    format!("{:?}", s.user_data.lock().await)
}

#[tokio::main]
async fn main() {
    // 1. Creating initial app state
    let mut initial_data = HashMap::new();
    // Adding usernames and passwords
    initial_data.insert(String::from("john_doe"), String::from("abc123"));
    initial_data.insert(String::from("mary_jane"), String::from("password"));

    // 2. Creating Router
    let app = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .merge(routes::auth::routes())
        .route("/debug", get(debug_command))
        .layer(
            // Attach CORS header for convenience
            // Enable all methods and all origins
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        )
        .with_state(AppState {
            user_data: Arc::new(Mutex::new(initial_data)),
        });

    // 3. Creating TcpListener, bound to port 8000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    // 4. Serving the app with the supplied listener (on port 8000)
    axum::serve(listener, app).await.unwrap();
}
