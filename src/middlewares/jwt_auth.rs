use std::future::Future;
use std::pin::Pin;

use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
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
    type Error = ErrorResponse;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .map(|h| h.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            let error = ErrorResponse::new("Missing token".to_string(), 401);
            return Box::pin(async move { Err(error) });
        }

        match JwtHandler::verify(token.unwrap().to_string()) {
            Ok(claims) => {
                let req = req.clone();
                Box::pin(async move {
                    let data = req.app_data::<web::Data<AppState>>().unwrap();
                    let req = HttpRequest::clone(&req);
                    let user = UserHandler::by_id(data.db.clone(), claims.user.id).await;
                    match user {
                        Ok(user) => {
                            if user.status == false {
                                return Err(ErrorResponse::new(
                                    "User are blocked".to_string(),
                                    401,
                                ));
                            }
                            req.extensions_mut().insert::<User>(user);
                            Ok(JwtMiddleware::default())
                        }
                        Err(error) => Err(error.status(401)),
                    }
                })
            }
            Err(error) => {
                return Box::pin(async move { Err(error.status(401)) });
            }
        }
    }
}
