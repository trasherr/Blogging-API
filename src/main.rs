use axum::{middleware, routing::get, Extension, Router} ;
use axum_prometheus::PrometheusMetricLayer;
use sea_orm::Database;
use tower_http::services::ServeDir;


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
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app: Router = Router::new()
    .merge(routes::user_routes::user_routes())
    .route_layer(middleware::from_fn(utils::guards::guard))
    .merge(routes::auth_routes::auth_routes())
    .merge(routes::home_routes::home_routes())
    .layer(Extension(db))
    .nest_service("/", ServeDir::new("public"))
    .route("/metrics", get(|| async move { metric_handle.render() }))
    .layer(prometheus_layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap()
}
