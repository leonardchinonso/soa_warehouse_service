use bson::doc;
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, error::Result as MongoResult, results::InsertOneResult, Collection};

use crate::model::stock::Stock;

#[derive(Clone)]
pub struct StockRepo {
    collection: Collection<Stock>,
}

impl StockRepo {
    pub fn new(collection: Collection<Stock>) -> Self {
        Self { collection }
    }

    // insert inserts a stock document in the database
    pub async fn insert(&self, stock: &Stock) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(stock, None).await
    }

    // get_by_client_id_and_product_id gets a stock from the database by
    // the client id and product id
    pub async fn get_by_client_id_and_product_id(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
    ) -> Result<Option<Stock>, Error> {
        let stock_result = self
            .collection
            .find_one(
                Some(doc! {"client_id": client_id, "product_id": product_id}),
                None,
            )
            .await?;
        Ok(stock_result)
    }

    // get_by_client_id_and_product_ids gets stocks from the database by
    // the client id and a list of product ids
    pub async fn get_by_client_id(
        &self,
        client_id: ObjectId,
    ) -> Result<Vec<Stock>, Error> {
        let filter = doc! {"client_id": client_id};
        let cursor = self.collection.find(filter, None).await?;
        let stocks: Vec<Stock> = cursor.try_collect().await?;
        Ok(stocks)
    }

    // update updates the stock in the database
    pub async fn update(&self, update: &Stock) -> MongoResult<UpdateResult> {
        let filter = doc! {"_id": update._id, "client_id": update.client_id, "product_id": update.product_id};
        let update_doc = doc! {"$set": {"quantity": update.get_quantity()}};
        self.collection.update_one(filter, update_doc, None).await
    }

    pub async fn delete_by_client_id_and_product_id(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
    ) -> MongoResult<DeleteResult> {
        let filter = doc! {"client_id": client_id, "product_id": product_id};
        self.collection.delete_one(filter, None).await
    }
}
