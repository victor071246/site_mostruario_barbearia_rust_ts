use axum::{
    routing::{get, post, patch, delete},
    Router,
    middleware
};
use crate::handlers::*;
use crate::auth::auth_middleware;

pub fn create_routes(state: AppState) -> Router {

    let public_routes = Router::new()
    .route("/auth/register", post(register))
    .route("/auth/login", post(login));

    let protected_routes = Router::new()
    .route("/items", get(list_items))
    .route("/items", post(create_item))
    .route("/items/:id", get(get_items))
    .route("/items/:id", patch(update_item))
    .route("/items/:id", delete(delete_items))
    .layer(middleware::from_fn(auth_middleware));

    Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .with_state(state)
}