use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub stock: i32,
    pub category: String,
    pub image_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct CheckAdminResponse {
    pub is_admin: bool,
}


#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub stock: i32,
    pub category: String,
    pub image_url: Option<String>
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub stock: Option<i32>,
    pub price: Option<rust_decimal::Decimal>,
    pub category: Option<String>,
    pub image_url: Option<String>, 
}