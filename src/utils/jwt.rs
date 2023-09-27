use axum::http::StatusCode;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::utils;




#[derive(Serialize,Deserialize)]
pub struct Cliams{
    pub exp: usize,
    pub iat: usize,
    pub email: String
}


pub fn encode_jwt(email: String) -> Result<String,StatusCode>{

    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Cliams{ iat: now.timestamp() as usize, exp: (now+expire).timestamp() as usize, email: email };
    let secret = (*utils::constants::TOKEN).clone();

    return encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });

}


pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>,StatusCode> {
    let secret = (*utils::constants::TOKEN).clone();
    let res: Result<TokenData<Cliams>, StatusCode> = decode(&jwt,&DecodingKey::from_secret(secret.as_ref()),&Validation::default())
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
    return res;
}