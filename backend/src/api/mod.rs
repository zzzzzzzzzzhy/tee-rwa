mod handlers;
mod routes;

use axum::Router;
use sqlx::PgPool;
use crate::config::Config;

pub fn create_router(db: PgPool, config: Config) -> Router {
    Router::new()
        .merge(routes::create_routes())
        .with_state((db, config))
}
