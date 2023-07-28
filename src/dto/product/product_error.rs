// use actix_web::{
//     http::{header::ContentType, StatusCode},
//     HttpResponse, ResponseError,
// };
// use std::fmt;

// // ProductError is a custom error for the Products API
// #[derive(Debug)]
// pub enum ProductError {
//     ProductCreationFailure,
//     ProductNotFound,
//     ProductUpdateFailure,
//     BadProductRequest,
//     InternalServerError,
// }

// // custom messages for product errors
// impl ProductError {
//     fn message(&self) -> String {
//         match self {
//             ProductError::ProductCreationFailure => String::from("cannot create product"),
//             ProductError::ProductNotFound => String::from("product not found"),
//             ProductError::ProductUpdateFailure => String::from("cannot update product"),
//             ProductError::BadProductRequest => String::from("bad request for product"),
//             ProductError::InternalServerError => String::from("internal server error"),
//         }
//     }
// }

// // implementing the Display trait for ProductError
// impl fmt::Display for ProductError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.message())
//     }
// }

// impl ResponseError for ProductError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::json())
//             .body(self.message())
//     }

//     fn status_code(&self) -> StatusCode {
//         match self {
//             ProductError::ProductCreationFailure => StatusCode::FAILED_DEPENDENCY,
//             ProductError::ProductNotFound => StatusCode::NOT_FOUND,
//             ProductError::ProductUpdateFailure => StatusCode::FAILED_DEPENDENCY,
//             ProductError::BadProductRequest => StatusCode::BAD_REQUEST,
//             ProductError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
