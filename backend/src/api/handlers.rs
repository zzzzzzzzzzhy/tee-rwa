use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::config::Config;
use crate::models::*;

pub async fn connect_wallet(
    State((db, _config)): State<(PgPool, Config)>,
    Json(payload): Json<ConnectWalletRequest>,
) -> Result<Json<ConnectWalletResponse>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (wallet_address)
        VALUES ($1)
        ON CONFLICT (wallet_address) DO UPDATE SET wallet_address = EXCLUDED.wallet_address
        RETURNING *
        "#
    )
    .bind(&payload.wallet_address)
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ConnectWalletResponse {
        user,
        message: "Wallet connected successfully".to_string(),
    }))
}

pub async fn get_user(
    State((db, _config)): State<(PgPool, Config)>,
    axum::extract::Path(wallet_address): axum::extract::Path<String>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE wallet_address = $1"
    )
    .bind(&wallet_address)
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
