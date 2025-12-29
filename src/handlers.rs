use axum::{
    Json, extract::{Path, State}, http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use sqlx::PgPool;
use serde_json::{json, Value};

use crate::{errors::AppError, models::*};
use crate::auth::*;
use crate::db::AppState;


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
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<impl IntoResponse, AppError> {

    // Busca usuário no banco
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash, created_at as \"created_at!\" FROM users WHERE username = $1",
        payload.username
    ).fetch_one(&state.pool).await.map_err(|error| match error {
        sqlx::Error::RowNotFound => AppError::new("User not found", StatusCode::NOT_FOUND),
        _ => AppError::new("Database error", StatusCode::INTERNAL_SERVER_ERROR),
    })?;

    // Verifica a senha
    let password_valid = verify_password(&payload.password, &user.password_hash).map_err(|_| AppError::new("Password verification failed", StatusCode::INTERNAL_SERVER_ERROR))?;

    if !password_valid {
        return Err(AppError::new("Invalid password", StatusCode::UNAUTHORIZED));
    }

    // 3. Gera token JWT
    let token = create_jwt(&user.username).to_owned().map_err(|_| AppError::new("Failed to create token", StatusCode::INTERNAL_SERVER_ERROR))?;

    let cookie = format!(
        "token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400",
        token
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((
        headers,
        Json(json!({
            "message": "Login successful",
            "user": { "username": user.username}
        }))
    ))
}


// Handlers de items

//GET /items 
pub async fn list_items(State(state): State<AppState>) -> Result<Json<Value>, AppError> {

    let items = sqlx::query_as!(
        Item,
        "SELECT id, name, description, price, category, image_url, stock, created_at as \"created_at!\", updated_at as \"updated_at!\" FROM items ORDER BY created_at DESC"
    ).fetch_all(&state.pool).await.map_err(|_| AppError::new("Failed to fetch items", StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(json!({
        "items": items,
        "count": items.len()
    })))
}

// GET /items/:id 
pub async fn get_items(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Item>, AppError> {

    let item = sqlx::query_as!(
        Item,
        "SELECT id, name, description, price, category, image_url, stock, created_at as \"created_at!\", updated_at as \"updated_at!\" FROM items WHERE id = $1",
        id
    ).fetch_one(&state.pool).await.map_err(|error| match error {
        sqlx::Error::RowNotFound => AppError::new("Item not found", StatusCode::NOT_FOUND),
        _ => AppError::new("Database error", StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    Ok(Json(item))
}

pub async fn create_item(State(state): State<AppState>, Json(payload): Json<CreateItemRequest> ) -> Result<Json<Value>, AppError> {

    let item = sqlx::query_as!(
        Item,
        "INSERT INTO items (name, description, price, category, image_url, stock)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, name, description, price, category, image_url, stock, created_at as \"created_at!\", updated_at as \"updated_at!\"
        ",
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.image_url,
        payload.stock
    ).fetch_one(&state.pool).await.map_err(|_| AppError::new("Failed to create item", StatusCode::INTERNAL_SERVER_ERROR))?;


    Ok(Json(json!({
        "message": "Item created successfully",
        "item": item
    })))
}

// PUT /items/:id
pub async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>
) -> Result<Json<Value>, AppError> {

    let current_item = sqlx::query_as!(
        Item,
        "SELECT id, name, description, price, category, image_url, stock, created_at as \"created_at!\", updated_at as \"updated_at!\" 
         FROM items WHERE id = $1",
        id
    ).fetch_one(&state.pool).await.map_err(|error| match error {
        sqlx::Error::RowNotFound => AppError::new("Item not found", StatusCode::NOT_FOUND),
        _ => AppError::new("Database error", StatusCode::INTERNAL_SERVER_ERROR)
    })?;
    
    //Usa os valores novos ou mantém os atuais em caso de erro
    let updated_item = sqlx::query_as!(
        Item,
        "UPDATE items
         SET name = $1, description = $2, price = $3, category = $4, image_url = $5, stock = $6, updated_at = NOW()
         WHERE id = $7
         RETURNING id, name, description, price, category, image_url, stock, created_at as \"created_at!\", updated_at as \"updated_at!\"",
        payload.name.unwrap_or(current_item.name),
        payload.description.or(current_item.description),
        payload.price.unwrap_or(current_item.price),
        payload.category.unwrap_or(current_item.category),
        payload.image_url.or(current_item.image_url),
        payload.stock.unwrap_or(current_item.stock),
        id
    ).fetch_one(&state.pool).await.map_err(|_| AppError::new("Failed to update item", StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(json!({
        "message": "Item update successfully",
        "item": updated_item
    })))
}

// DELETE /items/:id 
pub async fn delete_items (State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Value>, AppError> {

    let result = sqlx::query!(
        "DELETE FROM items WHERE id = $1 RETURNING id",
        id
    ).fetch_one(&state.pool).await.map_err(|error| match error {
        sqlx::Error::RowNotFound => AppError::new("Item not found", StatusCode::NOT_FOUND),
        _ => AppError::new("Database error", StatusCode::INTERNAL_SERVER_ERROR),
    })?;

    Ok(Json(json!({
        "message": "Item deleted successfully",
        "deleted_id": result.id
    })))
}