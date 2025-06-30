use actix_web::{web, HttpResponse, Result};
use solana_sdk::signature::Signer;
use base58::ToBase58;
use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::{Verifier, PublicKey, Signature};

use crate::models::{
    ApiResponse, SignMessageRequest, VerifyMessageRequest,
    SignatureResponse, VerificationResponse
};
use crate::utils::{parse_pubkey, parse_keypair};

pub async fn sign_message(req: web::Json<SignMessageRequest>) -> Result<HttpResponse> {
    if req.message.is_empty() || req.secret.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Missing required fields")));
    }

    let keypair = match parse_keypair(&req.secret) {
        Ok(kp) => kp,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response = SignatureResponse {
        signature: general_purpose::STANDARD.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message.clone(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn verify_message(req: web::Json<VerifyMessageRequest>) -> Result<HttpResponse> {
    let pubkey = match parse_pubkey(&req.pubkey) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid signature format"))),
    };

    if signature_bytes.len() != 64 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid signature length")));
    }

    let mut signature_array = [0u8; 64];
    signature_array.copy_from_slice(&signature_bytes);

    let verifying_key = match PublicKey::from_bytes(&pubkey.to_bytes()) {
        Ok(key) => key,
        Err(_) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid public key"))),
    };

    let signature = ed25519_dalek::Signature::from_bytes(&signature_array);
    // let is_valid = verifying_key.verify(req.message.as_bytes(), &signature).is_ok();
    let is_valid = match signature {
    Ok(sig) => verifying_key.verify(req.message.as_bytes(), &sig).is_ok(),
    Err(_) => false, // Invalid signature format means verification fails
};

    let response = VerificationResponse {
        valid: is_valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}