use actix_web::web::Json;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde_json::json;
use viman::establish_connection;
use viman::models::user::UserHander;
use viman::models::{ErrorResponse, LoginInfo, NewUser};

#[get("")]
async fn api() -> impl Responder {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");

    HttpResponse::Ok().json(json!({ "message": "Welcome to ".to_owned() + name + " api",
        "version": version,
    }))
}

#[get("/list")]
async fn user_list() -> impl Responder {
    let connection = &mut establish_connection();
    let all_users = UserHander::list(connection);

    match all_users {
        Ok(Some(all_users)) => {
            if !all_users.is_empty() {
                HttpResponse::Ok().json(Json(all_users))
            } else {
                HttpResponse::Ok().json(ErrorResponse::new("No users found.".to_string()))
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ErrorResponse::new("No users found.".to_string()))
        }
        Err(err) => HttpResponse::NotFound().json(Json(ErrorResponse::new(err.to_string()))),
    }
}

#[get("/{id}")]
async fn user_by_id(path: web::Path<i32>) -> impl Responder {
    let connection = &mut establish_connection();
    let _id = path.into_inner();

    let user = UserHander::by_id(connection, _id);

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(Json(user)),
        Ok(None) => HttpResponse::NotFound().json(Json(ErrorResponse::new("No data".to_string()))),
        Err(_) => HttpResponse::NotFound().json(Json(ErrorResponse::new("Error".to_string()))),
    }
}

#[delete("/{id}")]
async fn user_delete(path: web::Path<i32>) -> impl Responder {
    let connection = &mut establish_connection();
    let _id = path.into_inner();
    let result = UserHander::delete(connection, _id);
    match result {
        Ok(_) => HttpResponse::Ok().json(true),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
}

#[post("/register")]
async fn user_registeration(new_user: Json<NewUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let user = UserHander::create(connection, new_user.0);
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(Json(user)),
        Ok(None) => HttpResponse::NotFound().json(Json(ErrorResponse::new("No data".to_string()))),
        Err(_) => HttpResponse::NotFound().json(Json(ErrorResponse::new("Error".to_string()))),
    }
}

#[post("/login")]
async fn user_login(login_info: Json<LoginInfo>) -> impl Responder {
    let connection = &mut establish_connection();
    let login_result = UserHander::login(connection, login_info.0);
    match login_result {
        Ok(login_result) => HttpResponse::Ok().json(Json(login_result)),
        Err(error) => HttpResponse::NotFound().json(Json(error)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new().service(web::redirect("/", "/api")).service(
            web::scope("/api").service(api).service(
                web::scope("/user")
                    .service(user_list)
                    .service(user_registeration)
                    .service(user_login)
                    .service(user_by_id)
                    .service(user_delete),
            ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
