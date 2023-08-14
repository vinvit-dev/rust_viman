use std::env;
use std::ops::Deref;
use std::ptr::null;
use argonautica::Hasher;

use diesel::{PgConnection, Connection, RunQueryDsl, SelectableHelper, QueryDsl, ExpressionMethods, OptionalExtension, Insertable};
use diesel::pg::Pg;
use diesel::result::Error;
use dotenvy::dotenv;
use rocket::serde::json::Json;

use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("env variable DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, mut new_user: NewUser) -> Result<Option<User>, Error> {
    use crate::schema::users;
    dotenv().ok();

    let secret = env::var("SECRET_KEY")
        .expect("env variable SECRET_KEY must be set");

    let hesher = &mut Hasher::default();
    let hashed_password = hesher
        .with_password(new_user.password)
        .with_secret_key(secret)
        .hash()
        .unwrap();

   new_user.password = &hashed_password;

    diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .optional()
}

pub fn get_all_users(conn: &mut PgConnection) -> Result<Option<Vec<User>>, Error> {
    users
        .select(User::as_select())
        .load(conn)
        .optional()
}
pub fn get_user(conn: &mut PgConnection, _id: i32) -> Result<Option<User>, Error> {
    use self::schema::users::dsl::*;
    users
        .find(_id)
        .select(User::as_select())
        .first(conn)
        .optional()

}
