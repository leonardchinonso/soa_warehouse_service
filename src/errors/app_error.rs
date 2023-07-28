use actix_web::HttpResponse;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ErrorKind {
    InternalServerError,
    NotFound,
    FailedAction,
}

#[derive(Debug, Serialize)]
// AppError is a custom warehouse application error
pub struct AppError {
    pub message: String,
    pub kind: ErrorKind,
}

impl AppError {
    pub fn new(msg: &str, err_kind: ErrorKind) -> Self {
        Self {
            message: msg.to_string(),
            kind: err_kind,
        }
    }

    pub fn to_responder(self) -> HttpResponse {
        match self.kind {
            ErrorKind::InternalServerError => HttpResponse::InternalServerError().finish(),
            ErrorKind::NotFound => HttpResponse::BadRequest()
                .reason("resource not found")
                .json(self),
            ErrorKind::FailedAction => HttpResponse::BadRequest().json(self),
        }
    }
}

// implementing the Display trait for the custom error
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// implementing the Error for the custom error
impl Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}
