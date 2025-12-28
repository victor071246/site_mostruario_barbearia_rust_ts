use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub category: String,
    pub image_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub category: String,
    pub image_url: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub category: Option<String>,
    pub image_url: Option<String>, 
}