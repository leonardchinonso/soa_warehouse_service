use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use std::fmt;

// APIError is a custom error for the API
#[derive(Debug)]
pub enum APIError {
    CreationFailure,
    NotFound,
    UpdateFailure,
    BadRequest,
}

// custom messages for product errors
impl APIError {
    fn message(&self) -> String {
        match self {
            APIError::CreationFailure => String::from("cannot create document"),
            APIError::NotFound => String::from("document not found"),
            APIError::UpdateFailure => String::from("cannot update document"),
            APIError::BadRequest => String::from("bad request"),
        }
    }
}

// implementing the Display trait for APIError
impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.message())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            APIError::CreationFailure => StatusCode::FAILED_DEPENDENCY,
            APIError::NotFound => StatusCode::NOT_FOUND,
            APIError::UpdateFailure => StatusCode::FAILED_DEPENDENCY,
            APIError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
