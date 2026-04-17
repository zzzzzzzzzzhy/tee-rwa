use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::crypto::hash_data;

#[derive(Debug, Serialize)]
pub struct QuoteResponse {
    pub quote: String,
    pub tee_pubkey: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub encrypted_data: String,
}

#[derive(Debug, Serialize)]
pub struct EvaluateResponse {
    pub report_hash: Vec<u8>,
    pub credit_score: u32,
    pub risk_level: u8,
    pub signature: Vec<u8>,
    pub report_encrypted: String,
}

pub async fn get_quote(
    State(state): State<Arc<RwLock<crate::AppState>>>,
) -> Json<QuoteResponse> {
    let state = state.read().await;
    let pubkey = state.keypair.public_key_bytes();
    
    let quote = BASE64.encode(b"MOCK_QUOTE_SIMULATION_MODE");
    
    Json(QuoteResponse {
        quote,
        tee_pubkey: pubkey.to_vec(),
    })
}

pub async fn evaluate(
    State(state): State<Arc<RwLock<crate::AppState>>>,
    Json(_payload): Json<EvaluateRequest>,
) -> Json<EvaluateResponse> {
    let state = state.read().await;
    
    let credit_score = 750u32;
    let risk_level = 2u8;
    let report_data = format!("credit_score={},risk_level={}", credit_score, risk_level);
    let report_hash = hash_data(report_data.as_bytes());
    let signature = state.keypair.sign(&report_hash).unwrap();
    
    let report_encrypted = BASE64.encode(&report_data);
    
    Json(EvaluateResponse {
        report_hash: report_hash.to_vec(),
        credit_score,
        risk_level,
        signature: signature.to_vec(),
        report_encrypted,
    })
}
