use std::env;

use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::models::{Claims, ErrorResponse, UserLogin};

pub fn get_secret() -> String {
    env::var("SECRET_KEY").expect("env variable SECRET_KEY must be set")
}

pub fn get_secret_jwt() -> String {
    env::var("SECRET_JWT").expect("env variable SECRET_JWT must be set")
}

pub fn create_jwt(user: UserLogin) -> String {
    let claims = Claims {
        id: user.id,
        password: user.password,
        exp: Local::now().timestamp() + 24 * 3600,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret_jwt().as_ref()),
    )
    .unwrap()
}

pub fn verify_jwt(token: String) -> Result<Claims, ErrorResponse> {
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(get_secret_jwt().as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claims) => Ok(claims.claims),
        Err(error) => {
            let error = ErrorResponse::new(error.to_string());
            return Err(error);
        }
    }
}
