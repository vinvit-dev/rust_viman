use std::env;

use diesel::{PgConnection, Connection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;

use crate::models::{NewUser, User};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("env variable DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, username: &str, password: &str, email: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser {username, password, email};

    diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error creating new user")
}
