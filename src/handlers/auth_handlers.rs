use axum::{Json, response::IntoResponse, http::StatusCode, Extension};
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, Condition, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::{models::user_models::{CreateUserModel, LoginUserModel, UserModel, LoginUserResponseModel}, utils::{api_error::APIError, jwt::encode_jwt}};


pub async fn create_user_post(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>
) -> Result<(),APIError> {

    let user = entity::user::Entity::find()
    .filter(entity::user::Column::Email.eq(user_data.email.clone()))
    .one(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    if user != None {
        return  Err(APIError { message: "User exists".to_owned(), status_code:StatusCode::CONFLICT, error_code: Some(40) });
    }

    let user_model = entity::user::ActiveModel{
      name: Set(user_data.name.to_owned()),
      email: Set(user_data.email.to_owned()),
      password: Set(user_data.password.to_owned()),
      uuid: Set(Uuid::new_v4()),
      created_at: Set(Utc::now().naive_utc()) ,
        ..Default::default()
    };
    user_model.insert(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

   Ok(())

}

pub async fn login_user_post(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>
) -> Result<Json<LoginUserResponseModel>,APIError> {

    let user = entity::user::Entity::find()
    .filter(
        Condition::all()
            .add(entity::user::Column::Email.eq(user_data.email))
            .add(entity::user::Column::Password.eq(user_data.password))
    ).one(&db)
    .await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?;

    let token = encode_jwt(user.email)
    .map_err(|_| APIError { message: "Failed to login".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41) })?;

    Ok(Json(LoginUserResponseModel { token }))

}