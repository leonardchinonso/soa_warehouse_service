use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Collection};

use crate::model::product::Product;

#[derive(Clone)]
pub struct ProductRepo {
    collection: Collection<Product>,
}

impl ProductRepo {
    // new creates a product repository instance
    pub fn new(collection: Collection<Product>) -> Self {
        Self { collection }
    }

    // insert inserts a product in the database
    pub async fn insert(&self, product: &Product) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(product, None).await
    }

    // get_by_id retrieves a product from the database by id
    pub async fn get_by_id(
        &self,
        product_id: ObjectId,
        client_id: ObjectId,
    ) -> Result<Option<Product>, Error> {
        let product = self
            .collection
            .find_one(
                Some(doc! {"_id": product_id, "created_by": client_id}),
                None,
            )
            .await?;
        Ok(product)
    }

    // get_by_client_id retrieves all products created by the client
    pub async fn get_by_client_id(&self, client_id: ObjectId) -> Result<Vec<Product>, Error> {
        let filter = doc! {"created_by": client_id};
        let cursor = self.collection.find(filter, None).await?;
        let products: Vec<Product> = cursor.try_collect().await?;
        Ok(products)
    }

    // update updates a product in the database
    pub async fn update(
        &self,
        client_id: ObjectId,
        update: &Product,
    ) -> mongodb::error::Result<UpdateResult> {
        let filter = doc! {"_id": update._id, "created_by": client_id};
        let update_doc =
            doc! {"$set": {"name": update.name.clone(), "description": update.description.clone()}};
        self.collection.update_one(filter, update_doc, None).await
    }

    // delete_by_id deletes a product by id
    pub async fn delete_by_id(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
    ) -> mongodb::error::Result<DeleteResult> {
        let filter = doc! {"_id": product_id, "created_by": client_id};
        self.collection.delete_one(filter, None).await
    }
}
