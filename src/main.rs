mod controllers;
mod db;
mod auth;
mod models;

use actix_web::{web, App, HttpServer};
use std::env;
use env_logger;
use mongodb::{Client, options::ClientOptions};
use db::hactix_db::AppState;

async fn create_mongo_client() -> mongodb::error::Result<Client> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    client_options.app_name = Some("HelloActix".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mongo_client = create_mongo_client().await.expect("Failed to initialize MongoDB client");
    let app_state = web::Data::new(AppState::new(mongo_client));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controllers::hactix_controller::configure)
            .configure(controllers::auth_controller::configure)  // Novo controlador de autenticação
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
