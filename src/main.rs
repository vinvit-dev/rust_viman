use std::sync::Mutex;

use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};

use dotenvy::dotenv;
use env_logger::Env;
use services::{api::api_service, user::user_service};
use viman::{establish_connection, models::app::AppState};

mod middlewares;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let connection = establish_connection();

    let app_data = web::Data::new(AppState {
        db: Mutex::new(connection),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::redirect("/", "/api"))
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .service(
                web::scope("/api")
                    .service(web::scope("/user").configure(user_service))
                    .service(web::scope("").configure(api_service)),
            )
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
