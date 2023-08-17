#[macro_use]
extern crate rocket;

use std::env;
use chrono::{Local, Timelike};
use diesel::{Insertable, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use viman::{establish_connection, get_all_users, get_user, login, verify_password};
use viman::models::{Claims, JwtToken, LoginInfo, NewUser, User, UserLogin};
use viman::schema::users::dsl::users;
use viman::schema::users::username;

#[get("/")]
fn index() -> Redirect {
   Redirect::to("/api")
}
#[get("/")]
fn api() -> Value {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");

    json!({
        "message": "Welcome to ".to_owned() + name + " api",
        "version": version,
    })
}

#[get("/users")]
fn all_users() -> Result<Json<Vec<User>>, Status> {
    let connection = &mut establish_connection();
    let all_users = get_all_users(connection);

    match all_users {
        Ok(Some(all_users)) => Ok(Json(all_users)),
        Ok(None) => Err(Status::NoContent),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<_id>")]
fn user(_id: i32) -> Result<Json<User>, Status> {
    let connection = &mut establish_connection();

    let user = get_user(connection, _id);

    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/register", data="<new_user>")]
fn user_register(new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    let connection = &mut establish_connection();

    let user = viman::create_user(connection, new_user.0);
    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::UnprocessableEntity),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/login", data="<login_info>")]
fn user_login(login_info: Json<LoginInfo>) -> Result<Json<JwtToken>, Status> {
    let connection = &mut establish_connection();

    let login = login(connection,  login_info.clone().0);

    match login {
        Ok(Some(login)) => {
            if login.status == true && verify_password(&login_info.password, &login.password) {
                let claims = Claims {
                    id: login.id,
                    password: login.password,
                    expire: Local::now().timestamp() + 24 * 3600,
                };

                dotenv().ok();

                let secret = env::var("SECRET_JWT")
                    .expect("env variable SECRET_JWT must be set");

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

                Ok(Json(
                    JwtToken {token}
                ))
            } else {
                Err(Status::Forbidden)
            }
        },
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            api,
            all_users,
            user,
            user_register,
            user_login,
        ])
        .mount("/", routes![
            index
        ])
}