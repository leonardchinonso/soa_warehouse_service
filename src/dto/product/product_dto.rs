use serde::{Deserialize, Serialize};

use crate::errors::app_error::{AppError, ErrorKind};

#[derive(Deserialize, Serialize)]
// struct to aid extractor in extracting the product id
pub struct ClientId {
    pub client_id: String,
}

#[derive(Deserialize, Serialize)]
// struct to aid extractor in extracting the product id
pub struct ProductId {
    pub product_id: String,
}
// AddProductRequest represents the request body for adding a product
#[derive(Deserialize)]
pub struct AddProductRequest {
    pub name: String,
    pub description: String,
    pub quantity: i32,
}

impl AddProductRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.quantity < 1 {
            return Err(AppError::new(
                "quantity cannot be less than 1",
                ErrorKind::FailedAction,
            ));
        }
        Ok(())
    }
}

// AddProductResponse represents the request body for adding a product
#[derive(Serialize)]
pub struct AddProductResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quantity: i32,
}

impl AddProductResponse {
    pub fn new(id: String, name: String, description: String, quantity: i32) -> Self {
        Self {
            id,
            name,
            description,
            quantity,
        }
    }
}
