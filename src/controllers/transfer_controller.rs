use actix_web::{web, HttpResponse, Result};
use solana_sdk::system_instruction;
use base58::ToBase58;
use base64::{Engine as _, engine::general_purpose};
use spl_token::{
    instruction::transfer,
    ID as TOKEN_PROGRAM_ID,
};
use spl_associated_token_account::get_associated_token_address;

use crate::models::{
    ApiResponse, SendSolRequest, SendTokenRequest,
    SolTransferResponse, TokenTransferAccount, TokenTransferResponse
};
use crate::utils::parse_pubkey;

pub async fn send_sol(req: web::Json<SendSolRequest>) -> Result<HttpResponse> {
    let from = match parse_pubkey(&req.from) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let to = match parse_pubkey(&req.to) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    if req.lamports == 0 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Amount must be greater than 0")));
    }

    let instruction = system_instruction::transfer(&from, &to, req.lamports);

    let response = SolTransferResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn send_token(req: web::Json<SendTokenRequest>) -> Result<HttpResponse> {
    let destination = match parse_pubkey(&req.destination) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let mint = match parse_pubkey(&req.mint) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let owner = match parse_pubkey(&req.owner) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    if req.amount == 0 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Amount must be greater than 0")));
    }

    // Get associated token accounts
    let source_ata = get_associated_token_address(&owner, &mint);
    let dest_ata = get_associated_token_address(&destination, &mint);

    let instruction = transfer(
        &TOKEN_PROGRAM_ID,
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        req.amount,
    ).map_err(|e| format!("Failed to create transfer instruction: {}", e));

    let instruction = match instruction {
        Ok(inst) => inst,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let accounts: Vec<TokenTransferAccount> = instruction
        .accounts
        .iter()
        .map(|acc| TokenTransferAccount {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let response = TokenTransferResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}