use axum::{Router, routing::get, response::IntoResponse, http::StatusCode};
use chrono::Utc;
use sea_orm::{Database, DatabaseConnection, Set, ActiveModelTrait};
use entity::user;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    
    server().await;
}

async fn server(){


    let app: Router = Router::new()
    .route("/api/test",get(test))
    .route("/api/test/insert",get(create_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap()
}

async fn test() -> impl IntoResponse{
    println!("Test Api");

    (StatusCode::ACCEPTED,"Hey There")
}

async fn create_user() -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("postgres://trasherr:trasherr@localhost:5432/BlogDB").await.unwrap();

    let user_model = user::ActiveModel{
      name: Set("test".to_owned()),
      email: Set("test@gmail.com".to_owned()),
      password: Set("12345678".to_owned()),
      uuid: Set(Uuid::new_v4()),
      created_at: Set(Utc::now().naive_utc()) ,
        ..Default::default()
    };
    let usr = user_model.insert(&db).await.unwrap();


    (StatusCode::ACCEPTED,"Inserted")

}
