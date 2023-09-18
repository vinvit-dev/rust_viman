use actix_cors::Cors;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use viman::database::Database;

use crate::services::api::api_service;
use crate::services::user::user_service;
use dotenvy::dotenv;
use env_logger::Env;
use viman::{establish_connection, models::app::AppState};

pub mod middlewares;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let pool = establish_connection().await;

    let app_data = web::Data::new(AppState {
        db: Database {
            connection: pool.clone(),
        },
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(
                Cors::default()
                    .allow_any_method()
                    .allow_any_header()
                    .allowed_origin("http://127.0.0.1:8080"),
            )
            .service(web::redirect("/", "/api"))
            .service(
                web::scope("/api")
                    .service(web::scope("/user").configure(user_service))
                    .service(web::scope("").configure(api_service)),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
