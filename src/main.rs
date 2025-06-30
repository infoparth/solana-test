mod controllers;
mod routes;
mod models;
mod utils;

use actix_web::{App, HttpServer, middleware::Logger};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting Solana HTTP Server...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}