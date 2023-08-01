use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Stock is the model for stocks
#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub _id: ObjectId,
    pub client_id: ObjectId,
    pub product_id: ObjectId,
    quantity: i32,
}

impl Stock {
    // new returns a new stock object
    pub fn new(client_id: ObjectId, product_id: ObjectId, quantity: i32) -> Self {
        Self {
            _id: ObjectId::new(),
            client_id,
            product_id,
            quantity,
        }
    }

    // set_quantity sets the quantity of a stock object
    pub fn set_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
    }

    // get_quantity returns the quantity in a stock
    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}
