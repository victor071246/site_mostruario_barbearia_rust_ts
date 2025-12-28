use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use sqlx::PgPool;
use serde_json::{json, Value};

use crate::{errors::AppError, models::*};
use crate::auth::*;

// Estado compartilhado entre handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

// Rotas de autenticação

// POST /auth/register - Cadastrar admin
pub async fn register(State(state): State<AppState>, Json(payload): Json<CreateUserRequest>) -> Result <Json<Value>, AppError> {
    //Hash da senha
    let password_hash = hash_password(&payload.password).map_err(|_| AppError::new("Failed do hash string", StatusCode::LOCKED))?;

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
        Err(_) =>Err(AppError::new("User already exists", StatusCode::CONFLICT)),
    }
}

// POST /auth/login - Fazer login
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<Value>, AppError> {

    // Busca usuário no banco
    let user = sqlx::quer_as!(
        User,
        "SELECT id, username, password_hash, created_at FROM users WHERE username = $1",
        payload.
    ).fetch_one(&state.pool).await.map_err(|error| match error {
        sqlx::Error:RowNotFound => AppError::new("User not found", StatusCode::NOT_FOUND),
        _ => AppError::new("Database error", StatusCode::INTERNAL_SERVER_ERROR),
    })?;

    // Verifica a senha
    let password_valid = verify_password(&payload.password, hash)
}
