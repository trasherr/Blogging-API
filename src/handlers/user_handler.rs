use axum::{Json, response::IntoResponse, http::StatusCode, extract::Path, Extension};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::user_models::{UpdateUserModel, UserModel};


pub async fn update_user_put(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>
) -> impl IntoResponse {

    // let db: DatabaseConnection = Database::connect("postgres://trasherr:trasherr@localhost:5432/BlogDB").await.unwrap();
    
    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
    .filter(entity::user::Column::Uuid.eq(uuid))
    .one(&db)
    .await.unwrap().unwrap().into();

    user.name = Set(user_data.name);

    user.update(&db).await.unwrap();
    // db.close().await.unwrap();

    (StatusCode::ACCEPTED,"Updated")

}

pub async fn delete_user_delete(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>
)-> impl IntoResponse {
    // let db: DatabaseConnection = Database::connect("postgres://trasherr:trasherr@localhost:5432/BlogDB").await.unwrap();

    let user = entity::user::Entity::find().filter(entity::user::Column::Uuid.eq(uuid)).one(&db).await.unwrap().unwrap();

    entity::user::Entity::delete_by_id(user.id).exec(&db).await.unwrap();

    // db.close().await.unwrap();

    (StatusCode::ACCEPTED,"deleted")
}

pub async fn all_user_get(
    Extension(db): Extension<DatabaseConnection>
)-> impl IntoResponse{
    // let db: DatabaseConnection = Database::connect("postgres://trasherr:trasherr@localhost:5432/BlogDB").await.unwrap();

    let users: Vec<UserModel> = entity::user::Entity::find().all(&db).await.unwrap().into_iter().map(|item| UserModel{
        name: item.name,
        email: item.email,
        uuid: item.uuid,
        created_at: item.created_at,
    }).collect();

    // db.close().await.unwrap();
    (StatusCode::ACCEPTED,Json(users))

}