use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Stock is the model for stocks
#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub _id: ObjectId,
    pub product_id: ObjectId,
    pub client_id: ObjectId,
    pub quantity: i32,
}

impl Stock {
    pub fn new(product_id: ObjectId, client_id: ObjectId, quantity: i32) -> Self {
        Self {
            _id: ObjectId::new(),
            product_id,
            client_id,
            quantity,
        }
    }
}
