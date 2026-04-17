use anyhow::Result;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub tee_service_url: String,
    pub solana_rpc_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            tee_service_url: std::env::var("TEE_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8081".to_string()),
            solana_rpc_url: std::env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
        })
    }
}
