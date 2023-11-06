use axum::{Json, response::IntoResponse, http::StatusCode, extract::Path, Extension};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::{models::user_models::{UpdateUserModel, UserModel}, utils::api_error::APIError};


pub async fn update_user_put(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>
) -> Result<(),APIError> {

    
    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
    .filter(entity::user::Column::Uuid.eq(uuid))
    .one(&db)
    .await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?
    .into();

    user.name = Set(user_data.name);

    user.update(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(())

}

pub async fn delete_user_delete(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>
)-> Result<(),APIError> {

    let user = entity::user::Entity::find()
    .filter(entity::user::Column::Uuid.eq(uuid))
    .one(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?;

    entity::user::Entity::delete_by_id(user.id)
    .exec(&db)
    .await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(())
}

pub async fn all_user_get(
    Extension(db): Extension<DatabaseConnection>
)-> Result<Json<Vec<UserModel>>,APIError>{

    let users: Vec<UserModel> = entity::user::Entity::find().all(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    
    .into_iter().map(|item| UserModel{
        name: item.name,
        email: item.email,
        uuid: item.uuid,
        created_at: item.created_at,
    }).collect();

    Ok(Json(users))

}