use bson::Document;
use mongodb::Collection;
use mongodb::{error::Error, results::InsertOneResult};

use crate::dto::product::product_error::ProductError;
use crate::model::product::Product;
use crate::repository;

#[derive(Clone)]
pub struct ProductService {
    collection: Collection<Document>,
}

impl ProductService {
    pub fn new(collection: Collection<Document>) -> ProductService {
        ProductService { collection }
    }

    /// Insert product in mongo db
    pub async fn create(&self, product: &Product) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(product.to_insert_document(), None).await
    }

    pub fn get_product(product_id: i64) -> Result<Vec<Product>, ProductError> {
        let product = repository::product::get_product_by_id(product_id).unwrap();
        Ok(vec![product])
    }
}
