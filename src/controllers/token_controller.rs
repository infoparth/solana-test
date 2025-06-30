use actix_web::{web, HttpResponse, Result};
use base58::ToBase58;
use base64::{Engine as _, engine::general_purpose};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    ID as TOKEN_PROGRAM_ID,
};

use crate::models::{
    ApiResponse, CreateTokenRequest, MintTokenRequest, 
    AccountInfo, InstructionResponse
};
use crate::utils::parse_pubkey;

pub async fn create_token(req: web::Json<CreateTokenRequest>) -> Result<HttpResponse> {
    let mint_authority = match parse_pubkey(&req.mint_authority) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let mint = match parse_pubkey(&req.mint) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let instruction = initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint,
        &mint_authority,
        Some(&mint_authority),
        req.decimals,
    ).map_err(|e| format!("Failed to create initialize mint instruction: {}", e));

    let instruction = match instruction {
        Ok(inst) => inst,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let response = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn mint_token(req: web::Json<MintTokenRequest>) -> Result<HttpResponse> {
    let mint = match parse_pubkey(&req.mint) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let destination = match parse_pubkey(&req.destination) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let authority = match parse_pubkey(&req.authority) {
        Ok(pk) => pk,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let instruction = mint_to(
        &TOKEN_PROGRAM_ID,
        &mint,
        &destination,
        &authority,
        &[],
        req.amount,
    ).map_err(|e| format!("Failed to create mint instruction: {}", e));

    let instruction = match instruction {
        Ok(inst) => inst,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e))),
    };

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let response = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}