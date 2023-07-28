use bson::{doc, oid::ObjectId};

use crate::model::product::Product;
use mongodb::{error::Error, results::InsertOneResult, Collection};

#[derive(Clone)]
pub struct ProductRepo {
    collection: Collection<Product>,
}

impl ProductRepo {
    // new creates a product repository instance
    pub fn new(collection: Collection<Product>) -> Self {
        Self { collection }
    }

    // insert inserts a document in the database
    pub async fn insert(&self, product: &Product) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(product, None).await
    }

    // get_product_by_id retrieves a product from the database by id
    pub async fn get_product_by_id(&self, product_id: ObjectId) -> Result<Option<Product>, Error> {
        let product = self
            .collection
            .find_one(Some(doc! {"_id": product_id}), None)
            .await?;
        Ok(product)
    }
}
