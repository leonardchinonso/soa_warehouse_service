use mongodb::{error::Error, results::InsertOneResult, Collection};

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
}
