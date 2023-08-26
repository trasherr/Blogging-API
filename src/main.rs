use axum::{Router, routing::get, response::IntoResponse, http::StatusCode};


#[tokio::main]
async fn main() {
    
    server().await;
}

async fn server(){

    let app: Router = Router::new()
    .route("/api/test",get(test));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap()
}

async fn test() -> impl IntoResponse{
    println!("Test Api");

    (StatusCode::ACCEPTED,"Hey There")
}
