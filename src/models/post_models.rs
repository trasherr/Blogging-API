use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::user_models::UserMicroModel;


#[derive(Serialize,Deserialize,Default)]

pub struct PostModel {
    pub uuid: Uuid,
    pub text: String,
    pub image: String,
    pub title: String,
    pub user: Option<UserMicroModel>
}

#[derive(Serialize,Deserialize)]
pub struct CreatePostModel {
    pub text: String,
    pub image: String,
    pub title: String
}

impl From<(entity::post::Model,Option<entity::user::Model>)> for PostModel {
    fn from(value: (entity::post::Model,Option<entity::user::Model>)) -> Self {
        let u = value.1.unwrap();
        Self {
            uuid: value.0.uuid,
            text: value.0.text,
            image: value.0.image,
            title: value.0.title,
            user: Some(UserMicroModel { name: u.name, uuid: u.uuid }),
            ..Default::default()
        }
    }
}
