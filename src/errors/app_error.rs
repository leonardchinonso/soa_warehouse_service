use std::error::Error;
use std::fmt;

#[derive(Debug)]
// AppError is a custom warehouse application error
pub struct AppError {
    pub message: String,
}

enum errorKind {
    ServerError,
    LogicError,
}

impl AppError {
    pub fn new(msg: String) -> AppError {
        AppError { message: msg }
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
