use actix_web::http::StatusCode;
use sqlx::{Pool, Postgres};
use std::env;

pub mod models;
pub mod password;
pub mod utils;

pub fn status_code(code: u16) -> StatusCode {
    StatusCode::from_u16(code).unwrap()
}

pub async fn establish_connection() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set");
    Pool::<Postgres>::connect(database_url.as_str())
        .await
        .unwrap()
}

