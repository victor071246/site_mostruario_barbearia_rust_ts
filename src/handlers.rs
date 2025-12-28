use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use sqlx::PgPool;
use serde_json::{json, Value};

use crate::models::*;
use crate::auth::*;

// Estado compartilhado entre handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

// Rotas de autenticação

// POST /auth/register - Cadastrar admin
pub async fn register(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Result <Json<Value>, StatusCode> {
    //Hash da senha
    let password_hash = hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insere no banco
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        payload.username,
        password_hash
    ).fetch_one(&state.pool).await;

    let payload_username = payload.username;

    match  result {
        Ok(row) => Ok(Json(json!({
            "message": format!("Usuário {payload_username} criado com sucesso"),
            "user_id": row.id
        }))),
        Err(_) => Err(StatusCode::CONFLICT),
    }
} 