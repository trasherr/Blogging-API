use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, headers::{HeaderMapExt, Authorization, authorization::Bearer}};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use super::{api_error::APIError, jwt::decode_jwt};





pub async fn guard<T>(mut req: Request<T>, next: Next<T>) -> Result<Response,APIError> {

    let token = req.headers().typed_get::<Authorization<Bearer>>()
    .ok_or(APIError { message: "No Auth token found".to_owned(), status_code: StatusCode::BAD_REQUEST, error_code: Some(40)  })?.token().to_owned();

    let claim = decode_jwt(token)
    .map_err(|err| APIError { message: "Unauthorized".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41)  })?.claims;

    let db = req.extensions().get::<DatabaseConnection>()
    .ok_or(APIError { message: "Could not connect to database".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)  })?;

    let identity = entity::user::Entity::find()
    .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
    .one(db)
    .await.map_err(|err|  APIError { message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?
    .ok_or(APIError { message: "Unauthorized".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41)  });

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
} 