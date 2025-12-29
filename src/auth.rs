use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::env;

const JWT_SECRET_KEY: &str = "JWT_SECRET";


#[derive(Debug, Serialize, Deserialize)]

//Informações dentro do JWT
pub struct Claims {
    pub sub: String, // quem é o usuário (username) 
    pub exp: usize, // quando expira (timestamp Unix)
}

//Hash de senha
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Verifica senha contra hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

// Gera JWT token (válido por 24h)
pub fn create_jwt(username: &str) -> Result<String, jsonwebtoken:: errors::Error> {
    let secret = env::var(JWT_SECRET_KEY).expect(&format!("{} not in .env", JWT_SECRET_KEY));

    let expiration_time = chrono::Utc::now().checked_add_signed(chrono::Duration::hours(24)).expect("invalid timestamp").timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration_time,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

//Valida JWT token
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET should be in .env environment");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )?;

    Ok(token_data.claims)
}

//Middleware que valida JWT antes de chamar os handlers
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {

    // Tenta pegar do header Authorization ou do cookie
    let cookies = req.headers().get(header::COOKIE).and_then(|h| h.to_str().ok()).ok_or_else(|| AppError::new("Cookies missing", StatusCode::UNAUTHORIZED))?;

    let token = cookies.split(';').find_map(|cookie| {
        let cookie = cookie.trim();
        cookie.strip_prefix("token=")
    })
    .ok_or_else(|| AppError::new("Token not found in cookies", StatusCode::UNAUTHORIZED))?;

    // Valida o token
    let claims = validate_jwt(token).map_err(|_| AppError::new("Invalid or expired token", StatusCode::UNAUTHORIZED))?;


    // Guarda o username no request
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}