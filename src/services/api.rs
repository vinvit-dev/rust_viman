use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

pub fn api_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(api));
}

#[get("")]
async fn api() -> impl Responder {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");

    HttpResponse::Ok().json(json!({
        "message": "Welcome to ".to_owned() + name + " api",
        "version": version,
    }))
}
