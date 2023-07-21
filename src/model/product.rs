use crate::utils::tools;
use bson::{doc, Document};
use log::error;
use serde::{Deserialize, Serialize};

// pub struct NewProduct {
//     pub name: String,
//     pub description: String,
//     pub sku: String,
// }

// // NewProduct is the model for adding a new product
// impl NewProduct {
//     pub fn new(name: String, description: String) -> NewProduct {
//         let sku = tools::generate_random_alphanum(16).unwrap_or_else(|_| {
//             error!("Failed to generate random sku"); // log the error
//             panic!("error generating random sku"); // panic because it is a critical system error
//         });

//         let sku = tools::split_into_parts(sku, 4);

//         return NewProduct {
//             name,
//             description,
//             sku,
//         };
//     }
// }

// Product is the model for products
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub sku: String,
    // pub created_at: String,
    // pub updated_at: String,
}

impl Product {
    pub fn new(name: String, description: String) -> Self {
        let sku = tools::generate_random_alphanum(16).unwrap_or_else(|_| {
            error!("Failed to generate random sku"); // log the error
            panic!("error generating random sku"); // panic because it is a critical system error
        });

        let sku = tools::split_into_parts(sku, 4);
        let id = 0;

        return Self {
            id,
            name,
            description,
            sku,
        };
    }

    pub fn to_insert_document(&self) -> Document {
        let Product {
            id,
            name,
            description,
            sku,
        } = &self;
        doc! {
            "name": name,
            "description": description,
            "sku": sku,
        }
    }
}
