use axum::Router;
use axum::routing::{get, post};
use crate::api::handlers;

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/wallet/connect", post(handlers::connect_wallet))
        .route("/api/users/:wallet_address", get(handlers::get_user))
}
