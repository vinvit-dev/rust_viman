use crate::status_code;
use actix_web::body::BoxBody;
use actix_web::{error, HttpResponse, HttpResponseBuilder};
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}

impl ErrorResponse {
    pub fn new(error: String, status: u16) -> Self {
        Self { error, status }
    }
    pub fn status(self, status: u16) -> Self {
        Self {
            error: self.error,
            status,
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl error::ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(status_code(self.status)).json(self)
    }
}
