#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use std::env;
use viman::establish_connection;
use viman::models::user::UserHander;
use viman::models::{JwtToken, LoginInfo, NewUser, User};

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
    let all_users = UserHander::list(connection);

    match all_users {
        Ok(Some(all_users)) => Ok(Json(all_users)),
        Ok(None) => Err(Status::NoContent),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<_id>")]
fn user(_id: i32) -> Result<Json<User>, Status> {
    let connection = &mut establish_connection();

    let user = UserHander::by_id(connection, _id);

    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/register", data = "<new_user>")]
fn user_register(new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    let connection = &mut establish_connection();

    let user = UserHander::create(connection, new_user.0);
    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::UnprocessableEntity),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/login", data = "<login_info>")]
fn user_login(login_info: Json<LoginInfo>) -> Result<Json<JwtToken>, Status> {
    let connection = &mut establish_connection();

    let login = UserHander::login(connection, login_info.clone().0);

    match login {
        Ok(login) => Ok(Json(login)),
        Err(_) => Err(Status::Forbidden),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![api, all_users, user, user_register, user_login,],
        )
        .mount("/", routes![index])
}
