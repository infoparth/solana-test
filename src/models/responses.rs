use serde::Serialize;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SignatureResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerificationResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct SolTransferResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenTransferAccount {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct TokenTransferResponse {
    pub program_id: String,
    pub accounts: Vec<TokenTransferAccount>,
    pub instruction_data: String,
}