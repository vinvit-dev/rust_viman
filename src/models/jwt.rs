use std::ops::Add;

use crate::models::errors::ErrorResponse;
use crate::models::user::UserLogin;
use crate::utils::get_secret_jwt;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
    pub exp: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: UserLogin,
    pub exp: i64,
}

pub struct JwtHandler;

impl JwtHandler {
    pub fn create(user: UserLogin) -> Result<JwtToken, ErrorResponse> {
        let exp = Local::now().add(Duration::hours(1)).timestamp();
        let claims = Claims { user, exp };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret_jwt().as_ref()),
        );

        match token {
            Ok(token) => Ok(JwtToken { token, exp }),
            Err(error) => Err(ErrorResponse::new(error.to_string())),
        }
    }

    pub fn verify(token: String) -> Result<Claims, ErrorResponse> {
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(get_secret_jwt().as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(claims) => Ok(claims.claims),
            Err(error) => Err(ErrorResponse::new(error.to_string())),
        }
    }
}
