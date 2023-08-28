use actix_web::{routes, web};
use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use viman::models::{user::UserHander, AppState, ErrorResponse, LoginInfo, NewUser, User};

use crate::middlewares::jwt_auth::JwtMiddleware;

pub fn user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(user_list)
            .service(user_registeration)
            .service(user_login)
            .service(user_by_id)
            .service(user_page)
            .service(user_delete),
    );
}

#[routes]
#[get("/{id}")]
#[get("/{id}/")]
async fn user_list(data: web::Data<AppState>) -> impl Responder {
    let all_users = UserHander::list(&mut data.db.lock().unwrap());

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

#[routes]
#[get("/{id}")]
#[get("/{id}/")]
async fn user_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let _id = path.into_inner();

    let user = UserHander::by_id(&mut data.db.lock().unwrap(), _id);

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(Json(user)),
        Ok(None) => HttpResponse::NotFound().json(Json(ErrorResponse::new("No data".to_string()))),
        Err(_) => HttpResponse::NotFound().json(Json(ErrorResponse::new("Error".to_string()))),
    }
}

#[routes]
#[get("")]
#[get("/")]
async fn user_page(req: HttpRequest, _: JwtMiddleware) -> impl Responder {
    let extensions = req.extensions();
    let user = extensions.get::<User>();
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::Unauthorized().json(ErrorResponse::new("Not authorized".to_string())),
    }
}

#[routes]
#[delete("/{id}")]
#[delete("/{id}/")]
async fn user_delete(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let _id = path.into_inner();
    let result = UserHander::delete(&mut data.db.lock().unwrap(), _id);
    match result {
        Ok(_) => HttpResponse::Ok().json(true),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
}

#[routes]
#[post("/register")]
#[post("/register/")]
async fn user_registeration(new_user: Json<NewUser>, data: web::Data<AppState>) -> impl Responder {
    let user = UserHander::create(&mut data.db.lock().unwrap(), new_user.0);
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(Json(user)),
        Ok(None) => HttpResponse::NotFound().json(Json(ErrorResponse::new("No data".to_string()))),
        Err(_) => HttpResponse::NotFound().json(Json(ErrorResponse::new("Error".to_string()))),
    }
}

#[routes]
#[post("/login")]
#[post("/login/")]
async fn user_login(login_info: Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    let login_result = UserHander::login(&mut data.db.lock().unwrap(), login_info.0);
    match login_result {
        Ok(login_result) => HttpResponse::Ok().json(Json(login_result)),
        Err(error) => HttpResponse::NotFound().json(Json(error)),
    }
}
