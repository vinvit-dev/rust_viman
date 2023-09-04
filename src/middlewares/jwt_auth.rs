use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, http, web, FromRequest, HttpMessage};
use viman::models::app::AppState;
use viman::models::errors::ErrorResponse;
use viman::models::jwt::JwtHandler;
use viman::models::user::{User, UserHandler};

pub struct JwtMiddleware;

impl Default for JwtMiddleware {
    fn default() -> Self {
        Self {}
    }
}

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();
        let token = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .map(|h| h.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            let error = ErrorResponse::new("Missing token".to_string());
            return ready(Err(ErrorUnauthorized(error)));
        }

        match JwtHandler::verify(token.unwrap().to_string()) {
            Ok(claims) => {
                let user = UserHandler::by_id(&mut data.db.lock().unwrap(), claims.user.id)
                    .unwrap()
                    .unwrap();
                req.extensions_mut().insert::<User>(user.clone());
                ready(Ok(JwtMiddleware::default()))
            }
            Err(error) => {
                return ready(Err(ErrorUnauthorized(error)));
            }
        }
    }
}
