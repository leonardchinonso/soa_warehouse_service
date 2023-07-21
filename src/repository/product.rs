use crate::{
    dto::product::product_error::ProductError,
    errors::app_error::AppError,
    model::{
        product::Product,
    },
};

pub fn get_product_by_id(product_id: i64) -> Result<Product, AppError> {
    // let product = products.find(product_id).first(connection).ok();

    // match product {
    //     Some(product) => Ok(product),
    //     None => Err(AppError::new("product not found".to_string()))
    // }

    // Ok(Product{name: "".to_string(), description: "".to_string(), sku: "".to_string(), id: todo!() })
    todo!()
}
