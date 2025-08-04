use axum::Router;
use axum::routing::get;
use axum_diesel_example::routes;
use axum_diesel_example::state::AppState;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // 1. Creating initial app state
    // TODO : diesel init here

    // 2. Creating Router
    let app = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .merge(routes::auth::routes())
        .merge(routes::transactions::routes())
        .merge(routes::transactions_ws::routes())
        .merge(routes::user::routes())
        .layer(
            // Attach CORS header for convenience
            // Enable all methods and all origins
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        )
        .with_state(AppState {});

    // 3. Creating TcpListener, bound to port 8000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    // 4. Serving the app with the supplied listener (on port 8000)
    axum::serve(listener, app).await.unwrap();
}
