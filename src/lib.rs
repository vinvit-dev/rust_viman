use std::env;
use std::ops::Deref;
use argonautica::Hasher;

use diesel::{PgConnection, Connection, RunQueryDsl, SelectableHelper, QueryDsl, ExpressionMethods, OptionalExtension, Insertable};
use diesel::result::Error;
use dotenvy::dotenv;
use rocket::form::validate::Len;

use crate::models::{LoginInfo, NewUser, User};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, password, username};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("env variable DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn hash_password(password_to_hash: String) -> String {
    dotenv().ok();

    let secret = env::var("SECRET_KEY")
        .expect("env variable SECRET_KEY must be set");

    let hesher = &mut Hasher::default();
    let hashed_password = hesher
        .with_password(password_to_hash)
        .with_secret_key(secret)
        .hash()
        .unwrap();

    hashed_password
}

pub fn create_user(conn: &mut PgConnection, mut new_user: NewUser) -> Result<Option<User>, Error> {
    use crate::schema::users;

    let check_user = users
        .filter(email.eq(&new_user.email))
        .or_filter(username.eq(&new_user.username))
        .select(User::as_select())
        .load(conn)
        .optional().unwrap();

    if check_user.len() > 0 {
        return Err(panic!("User with this name or email already exist!"));
    }

    new_user.password = hash_password(new_user.password);

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

pub fn user_login(conn: &mut PgConnection, login_info: LoginInfo) -> Result<Option<User>, Error> {
    let hashed_password = hash_password(login_info.password);
    users
        .filter(username.eq(login_info.username))
        .filter(password.eq(hashed_password))
        .select(User::as_select())
        .first(conn)
        .optional()
}
