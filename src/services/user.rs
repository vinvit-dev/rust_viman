use crate::middlewares::jwt_auth::JwtMiddleware;
use actix_web::{delete, get, post, web, ResponseError};
use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use viman::models::app::AppState;
use viman::models::errors::ErrorResponse;
use viman::models::user::{LoginInfo, NewUser, User, UserHandler};

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
    let list = UserHandler::list(data.db.clone()).await;
    match list {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(error) => error.error_response(),
    }
}

#[get("/{id}")]
async fn user_by_id(
    path: web::Path<i32>,
    data: web::Data<AppState>,
    _: JwtMiddleware,
) -> impl Responder {
    let _id = path.into_inner();

    let user = UserHandler::by_id(data.db.clone(), _id).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => error.error_response(),
    }
}

#[delete("/{id}")]
async fn user_delete(
    path: web::Path<i32>,
    data: web::Data<AppState>,
    _: JwtMiddleware,
) -> impl Responder {
    let _id = path.into_inner();
    let result = UserHandler::delete(data.db.clone(), _id).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => error.error_response(),
    }
}

#[get("/me")]
async fn user_page(req: HttpRequest, _: JwtMiddleware) -> impl Responder {
    let extensions = req.extensions();
    let user = extensions.get::<User>();
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => ErrorResponse::new("Not authorized".to_string(), 401).error_response(),
    }
}

#[post("/register")]
async fn user_register(new_user: Json<NewUser>, data: web::Data<AppState>) -> impl Responder {
    let user = UserHandler::create(data.db.clone(), new_user.0).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => error.error_response(),
    }
}

#[post("/login")]
async fn user_login(login_info: Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    let login_result = UserHandler::login(data.db.clone(), login_info.0).await;
    match login_result {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(error) => error.error_response(),
    }
}
