use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, http, web, FromRequest, HttpMessage};
use viman::{
    models::{user::UserHander, AppState, ErrorResponse, User},
    utils::verify_jwt,
};

pub struct JwtMiddleware {
    pub user: User,
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
        println!("{}", token.clone().unwrap().to_string());

        match verify_jwt(token.unwrap().to_string()) {
            Ok(claims) => {
                let user = UserHander::by_id(&mut data.db.lock().unwrap(), claims.id)
                    .unwrap()
                    .unwrap();
                req.extensions_mut().insert::<User>(user.clone());
                ready(Ok(JwtMiddleware { user }))
            }
            Err(error) => {
                return ready(Err(ErrorUnauthorized(error)));
            }
        }
    }
}
