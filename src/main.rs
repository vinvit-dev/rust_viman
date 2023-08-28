use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

use dotenvy::dotenv;
use services::{api::api_service, user::user_service};
use viman::{establish_connection, models::AppState};

mod middlewares;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let connection = establish_connection();

    let app_data = web::Data::new(AppState {
        db: Mutex::new(connection),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::redirect("/", "/api"))
            .service(
                web::scope("/api")
                    .configure(api_service)
                    .configure(user_service),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
