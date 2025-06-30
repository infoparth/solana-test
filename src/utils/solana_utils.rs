use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;
use base58::FromBase58;

pub fn parse_pubkey(pubkey_str: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(pubkey_str).map_err(|_| "Invalid public key format".to_string())
}

pub fn parse_keypair(secret_str: &str) -> Result<Keypair, String> {
    let secret_bytes = secret_str.from_base58()
    .map_err(|_| "Invalid base58 encoding")?;
    
    if secret_bytes.len() != 64 {
        return Err("Invalid secret key length".to_string());
    }

    Keypair::from_bytes(&secret_bytes)
        .map_err(|_| "Invalid secret key".to_string())
}