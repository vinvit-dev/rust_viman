use actix_web::http::StatusCode;
use diesel::{Connection, PgConnection};
use std::env;

pub mod models;
pub mod password;
pub mod schema;
pub mod utils;

pub fn status_code(code: u16) -> StatusCode {
    StatusCode::from_u16(code).unwrap()
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
