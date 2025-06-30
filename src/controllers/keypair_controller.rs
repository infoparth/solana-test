use actix_web::{HttpResponse, Result};
use solana_sdk::signature::{Keypair, Signer};
use base58::ToBase58;

use crate::models::{ApiResponse, KeypairResponse};

pub async fn generate_keypair() -> Result<HttpResponse> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = keypair.to_bytes().to_base58();

    Ok(HttpResponse::Ok().json(ApiResponse::success(KeypairResponse {
        pubkey,
        secret,
    })))
}