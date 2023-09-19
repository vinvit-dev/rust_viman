use crate::middlewares::jwt_auth::JwtMiddleware;
use actix_web::{delete, get, post, web, ResponseError};
use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use viman::models::app::AppState;
use viman::models::balance::{BalanceHandler, NewBalance};
use viman::models::errors::ErrorResponse;
use viman::models::user::{LoginInfo, NewUser, User, UserHandler};

pub fn user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(user_register)
            .service(user_login)
            .service(user_page)
            .service(new_user_balance)
            .service(user_balances)
            .service(user_delete),
    );
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

#[get("")]
async fn user_page(req: HttpRequest, _: JwtMiddleware) -> impl Responder {
    let extensions = req.extensions();
    let user = extensions.get::<User>();
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => ErrorResponse::new("Not authorized".to_string(), 401).error_response(),
    }
}

#[post("/balance/create")]
async fn new_user_balance(
    req: HttpRequest,
    _: JwtMiddleware,
    new_balance: Json<NewBalance>,
    data: web::Data<AppState>,
) -> impl Responder {
    let extensions = req.extensions();
    let user = extensions.get::<User>().unwrap();
    let balance =
        BalanceHandler::crate_balance_for_user(data.db.clone(), user.clone(), new_balance.0).await;
    match balance {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => error.error_response(),
    }
}

#[get("/balances")]
async fn user_balances(
    req: HttpRequest,
    _: JwtMiddleware,
    data: web::Data<AppState>,
) -> impl Responder {
    let extensions = req.extensions();
    let user = extensions.get::<User>().unwrap();
    let balances = BalanceHandler::get_user_balances(data.db.clone(), user.clone()).await;
    match balances {
        Ok(balances) => HttpResponse::Ok().json(balances),
        Err(error) => error.error_response(),
    }
}
