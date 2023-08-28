use std::{fmt, sync::Mutex};

use crate::schema::users;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

pub mod user;

pub struct AppState {
    pub db: Mutex<PgConnection>,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub status: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UserLogin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub password: String,
    pub exp: i64,
}
