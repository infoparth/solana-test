use actix_web::{web, web::ServiceConfig};

use crate::controllers::{
    generate_keypair, create_token, mint_token, sign_message, 
    verify_message, send_sol, send_token
};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/keypair", web::post().to(generate_keypair))
            .route("/token/create", web::post().to(create_token))
            .route("/token/mint", web::post().to(mint_token))
            .route("/message/sign", web::post().to(sign_message))
            .route("/message/verify", web::post().to(verify_message))
            .route("/send/sol", web::post().to(send_sol))
            .route("/send/token", web::post().to(send_token))
    );
}