use crate::utils::tools;
use bson::oid::ObjectId;
use log::error;
use serde::{Deserialize, Serialize};

// Product is the model for products
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    sku: String,
    created_by: ObjectId,
}

// ProductQuantityResponse is the response body for getting a product with its quantity
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductQuantityResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    sku: String,
    pub quantity: i32,
}

impl Product {
    // new creates a new Product and assigns an sku and an _id value as the primary key
    pub fn new(name: String, description: String, client_id: ObjectId) -> Self {
        let sku = tools::generate_random_alphanum(16).unwrap_or_else(|_| {
            error!("Failed to generate random sku"); // log the error
            panic!("error generating random sku"); // panic because it is a critical system error
        });

        let sku = tools::split_into_parts(sku, 4);

        return Self {
            _id: ObjectId::new(),
            name,
            description,
            sku,
            created_by: client_id,
        };
    }

    // get_sku returns the value of the sku field
    pub fn get_sku(&self) -> String {
        self.sku.clone()
    }

    // to_product_quantity_response converts a product to a ProductQuantityResponse
    pub fn to_product_quantity_response(&self, quantity: i32) -> ProductQuantityResponse {
        ProductQuantityResponse {
            id: self._id.to_hex(),
            name: self.name.clone(),
            description: self.description.clone(),
            sku: self.sku.clone(),
            quantity,
        }
    }
}
