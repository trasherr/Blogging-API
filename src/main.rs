use axum::{Router, Extension, middleware} ;
use sea_orm::Database;


mod models;
mod routes;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    
    server().await;
}

async fn server(){

    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = Database::connect(conn_str).await.expect("Failed to connect to db");

    let app: Router = Router::new()
    .merge(routes::user_routes::user_routes())
    .route_layer(middleware::from_fn(utils::guards::guard))
    .merge(routes::auth_routes::auth_routes())
    .merge(routes::home_routes::home_routes())
    .layer(Extension(db));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap()
}