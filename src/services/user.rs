use actix_web::{delete, get, post, web};
use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use viman::models::app::AppState;
use viman::models::errors::ErrorResponse;
use viman::models::user::{LoginInfo, NewUser, User, UserHandler};

use crate::middlewares::jwt_auth::JwtMiddleware;

pub fn user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(user_register)
            .service(user_login)
            .service(user_list)
            .service(user_page)
            .service(user_by_id)
            .service(user_delete),
    );
}

#[get("/list")]
async fn user_list(data: web::Data<AppState>, _: JwtMiddleware) -> impl Responder {
    let all_users = UserHandler::list(&mut data.db.lock().unwrap());

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
async fn user_by_id(
    path: web::Path<i32>,
    data: web::Data<AppState>,
    _: JwtMiddleware,
) -> impl Responder {
    let _id = path.into_inner();

    let user = UserHandler::by_id(&mut data.db.lock().unwrap(), _id);

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(Json(user)),
        Ok(None) => HttpResponse::NotFound().json(Json(ErrorResponse::new("No data".to_string()))),
        Err(_) => HttpResponse::NotFound().json(Json(ErrorResponse::new("Error".to_string()))),
    }
}

#[delete("/{id}")]
async fn user_delete(
    path: web::Path<i32>,
    data: web::Data<AppState>,
    _: JwtMiddleware,
) -> impl Responder {
    let _id = path.into_inner();
    let result = UserHandler::delete(&mut data.db.lock().unwrap(), _id);
    match result {
        Ok(_) => HttpResponse::Ok().json(true),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
}

#[get("/me")]
async fn user_page(req: HttpRequest, _: JwtMiddleware) -> impl Responder {
    let extensions = req.extensions();
    let user = extensions.get::<User>();
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::Unauthorized().json(ErrorResponse::new("Not authorized".to_string())),
    }
}

#[post("/register")]
async fn user_register(new_user: Json<NewUser>, data: web::Data<AppState>) -> impl Responder {
    let user = UserHandler::create(&mut data.db.lock().unwrap(), new_user.0);
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(Json(user)),
        Ok(None) => HttpResponse::NotFound().json(Json(ErrorResponse::new("No data".to_string()))),
        Err(_) => HttpResponse::NotFound().json(Json(ErrorResponse::new("Error".to_string()))),
    }
}

#[post("/login")]
async fn user_login(login_info: Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    let login_result = UserHandler::login(&mut data.db.lock().unwrap(), login_info.0);
    match login_result {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(error) => HttpResponse::Unauthorized().json(error),
    }
}
