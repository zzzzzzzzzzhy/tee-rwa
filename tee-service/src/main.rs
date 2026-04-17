mod api;
mod crypto;

use axum::{Router, routing::{get, post}};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use crypto::TeeKeyPair;

pub struct AppState {
    pub keypair: TeeKeyPair,
}

impl AppState {
    fn new() -> anyhow::Result<Self> {
        let keypair = TeeKeyPair::generate()?;
        Ok(Self { keypair })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = Arc::new(RwLock::new(AppState::new()?));
    
    let app = Router::new()
        .route("/quote", get(api::get_quote))
        .route("/evaluate", post(api::evaluate))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("TEE service listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
