mod auth;
mod db;
mod errors;
mod models;
mod handlers;
mod routes;

use std::env;
use dotenv::dotenv;
use crate::{db::{AppState, create_pool}, routes::create_routes};

const DATABASE_URL_KEY: &str = "DATABASE_URL";

#[tokio::main]
async fn main() {
    // Carrega as variáveis do dotenv
    dotenv().ok();

    let database_url = env::var(DATABASE_URL_KEY).expect(&format!("{} must be set in .env", DATABASE_URL_KEY));


    let pool = create_pool(&database_url).await.expect("Failed to connect to database");

    // Cria o AppState com a pool
    let state = AppState { pool };

    let app = create_routes(state);

    // Define o endereço do servidor
    let addr = "0.0.0.0:3000";
    let string = format!("Server running on http://{addr}");
    println!("{}", string);

    // Cria o listener TCP
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind address to listener");

    // Inicia o servidor Axum
    axum::serve(listener, app).await.expect("Failed to start server");
}