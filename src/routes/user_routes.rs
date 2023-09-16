use axum::routing::{delete, put, get};
use axum::{Router, http::Method};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::user_handler;

pub fn user_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/user/:uuid/update",put(user_handler::update_user_put))
    .route("/api/user/:uuid/delete",delete(user_handler::delete_user_delete))
    .route("/api/user/all",get(user_handler::all_user_get))
    .layer(cors);
    router
}