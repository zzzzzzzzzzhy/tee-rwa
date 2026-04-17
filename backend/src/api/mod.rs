mod handlers;

use axum::Router;
use axum::routing::{get, post};
use sqlx::PgPool;
use crate::config::Config;

pub fn create_router(db: PgPool, config: Config) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/wallet/connect", post(handlers::connect_wallet))
        .route("/api/users/{wallet_address}", get(handlers::get_user))
        .with_state((db, config))
}
