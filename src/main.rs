#[macro_use]
extern crate rocket;

use diesel::Insertable;
use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use viman::{establish_connection, get_all_users, get_user};
use viman::models::{NewUser, User};

#[get("/")]
fn hello() -> Value {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");

    json!({
        "message": "Welcome to ".to_owned() + name + " api",
        "version": version,
    })
}

#[get("/users")]
fn users() -> Result<Json<Vec<User>>, Status> {
    let connection = &mut establish_connection();
    let users = get_all_users(connection);

    match users {
        Ok(Some(users)) => Ok(Json(users)),
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

#[post("/user", data="<new_user>")]
fn create_user(new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    let connection = &mut establish_connection();

    let user = viman::create_user(connection, new_user.0);
    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::UnprocessableEntity),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![
        hello,
        users,
        user,
        create_user,
    ])
}