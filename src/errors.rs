use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

//Estrutura de resposta JSON de erro
#[derive(Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

// Erro customizado
#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

impl AppError {
    pub fn new(message: &str, status_code: StatusCode) -> Self {
        AppError {
            message: message.to_string(),
            status_code,
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            status: "error".to_string(),
            message: self.message,
        });

        (self.status_code, body).into_response()
    }
}

// Converte erros de SQLx
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::new("User not found", StatusCode::NOT_FOUND),
            sqlx::Error::Database(database_error) => AppError::new(&format!(" Database error {}", database_error), StatusCode::INTERNAL_SERVER_ERROR),
            _ => AppError::new("Sqlx not mapped internal error", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}