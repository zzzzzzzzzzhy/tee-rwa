use axum::Router;
use axum::routing::{get, post};

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/health", get(|| async { "ok" }))
}
