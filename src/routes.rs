use axum::{
    routing::{get, post, patch, delete},
    Router
};
use crate::handlers::*;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // Autenticação
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))

        // Items (CRUD completo)
        .route("/items", get(list_items))
        .route("/items", post(create_item))
        .route("/items/:id", get(get_items))
        .route("/items/:id", patch(update_item))
        .route("/items/:id", delete(delete_items))

        .with_state(state)
}