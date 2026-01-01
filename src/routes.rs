use axum::{
    routing::{get, post, patch, delete},
    Router,
    middleware
};
use tower_http::services::ServeDir;
use crate::handlers::*;
use crate::auth::auth_middleware;
use crate::db::AppState;

pub fn create_routes(state: AppState) -> Router {

    let public_routes = Router::new()
    .route("/auth/register", post(register))
    .route("/auth/login", post(login))
    .route("/auth/check-admin", get(check_user_is_admin))
    .route("/items", get(list_items))
    .route("/items/:id", get(get_items));


    let protected_routes = Router::new()
    .route("/items", post(create_item))
    .route("/items/:id", patch(update_item))
    .route("/items/:id", delete(delete_items))
    .layer(middleware::from_fn(auth_middleware));

    Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .nest_service("/uploads", ServeDir::new("uploads"))
    .with_state(state)
}