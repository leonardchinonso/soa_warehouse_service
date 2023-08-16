use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::{AppError, ErrorKind},
    model::product::ProductResponse,
};

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

#[derive(Deserialize, Serialize)]
// struct to aid extractor in extracting the product id and client id
pub struct ClientIdProductId {
    pub client_id: String,
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
    pub sku: String,
    pub quantity: i32,
}

// AddProductResponse represents the response body for adding a product
impl AddProductResponse {
    pub fn new(id: String, name: String, description: String, sku: String, quantity: i32) -> Self {
        Self {
            id,
            name,
            description,
            sku,
            quantity,
        }
    }
}

// GetProductResponse represents the request body for getting a product
#[derive(Serialize)]
pub struct GetProductResponse {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl GetProductResponse {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

// GetProductsResponse represents the request body for getting all products
#[derive(Serialize)]
pub struct GetProductsResponse {
    pub products: Vec<ProductResponse>,
}

// GetProductsResponse represents the response body for retrieving a product
impl GetProductsResponse {
    pub fn new(products: Vec<ProductResponse>) -> Self {
        Self { products }
    }
}

// UpdateProductRequest represents the request body for updating a product
#[derive(Deserialize)]
pub struct UpdateProductRequest {
    pub name: String,
    pub description: String,
}

// UpdateProductRequest represents the response body for updating a product
#[derive(Serialize)]
pub struct UpdateProductResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sku: String,
}

// UpdateProductResponse represents the response body for updating a product
impl UpdateProductResponse {
    pub fn new(id: String, name: String, description: String, sku: String) -> Self {
        Self {
            id,
            name,
            description,
            sku,
        }
    }
}

// SetProductQuantityRequest represents the request body for updating a product quantity
#[derive(Deserialize)]
pub struct SetProductQuantityRequest {
    pub quantity: i32,
}

// SetProductQuantityResponse represents the response body for updating a product quantity
#[derive(Serialize)]
pub struct SetProductQuantityResponse {
    pub quantity: i32,
}

impl SetProductQuantityResponse {
    pub fn new(quantity: i32) -> Self {
        Self { quantity }
    }
}

// CheckAvailabilityRequest represents the request query for checking the availability of a product
#[derive(Deserialize)]
pub struct CheckAvailabilityRequest {
    pub number: i32,
}

// ProductQuantityRequest represents the request body for processing an order
// and checking the availability of multiple products
#[derive(Debug, Deserialize)]
pub struct ProductQuantityRequest {
    pub product_id: String,
    pub quantity: i32,
}

/// Data Transfer Objects not involving API requests
pub struct ProductQuantity {
    pub product_id: ObjectId,
    pub quantity: i32,
}

impl ProductQuantity {
    pub fn new() -> Self {
        Self {
            product_id: ObjectId::new(),
            quantity: 0,
        }
    }
}
