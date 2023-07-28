use log::error;

use bson::oid::ObjectId;

use crate::{
    errors::app_error::{AppError, ErrorKind},
    model::{product::Product, stock::Stock},
    repository::{product_repo::ProductRepo, stock_repo::StockRepo},
};

#[derive(Clone)]
pub struct ProductService {
    product_repo: ProductRepo,
    stock_repo: StockRepo,
}

impl ProductService {
    pub fn new(product_repo: ProductRepo, stock_repo: StockRepo) -> ProductService {
        ProductService {
            product_repo,
            stock_repo,
        }
    }

    // create implements the business logic for creating a product
    pub async fn create(
        &self,
        product: &Product,
        client_id: ObjectId,
        quantity: i32,
    ) -> Result<(), AppError> {
        // insert the product in the database, get the insertion id and return an error if any
        let optional_product_id = match self.product_repo.insert(product).await {
            Ok(product_result) => product_result.inserted_id.as_object_id(),
            Err(err) => {
                error!("Error inserting product: {:?}", err);
                return Err(AppError::new(
                    "cannot create product",
                    ErrorKind::InternalServerError,
                ));
            }
        };

        // ensure product_id is valid
        let product_id = match optional_product_id {
            Some(product_id) => product_id,
            None => {
                error!("Error inserting product: cannot retrieve inserted id");
                return Err(AppError::new(
                    "cannot create product",
                    ErrorKind::InternalServerError,
                ));
            }
        };

        // // try to get the inserted product from the database
        // let product = match self.get_product(product_id).await {
        //     Ok(product) => product,
        //     Err(err) => {
        //         error!("Error retrieving inserted product: {:?}", err);
        //         return Err(AppError::new("cannot create product", ErrorKind::InternalServerError));
        //     }
        // };

        // insert the client_id, product_id and quantity in stock
        let stock = Stock::new(product_id, client_id, quantity);
        if let Err(err) = self.stock_repo.insert(&stock).await {
            error!("Error creating stock: {:?}", err);
            return Err(AppError::new(
                "cannot create product",
                ErrorKind::InternalServerError,
            ));
        }

        Ok(())
    }

    // get_product gets a product from the application storage
    pub async fn get_product(&self, product_id: ObjectId) -> Result<Product, AppError> {
        match self.product_repo.get_product_by_id(product_id).await {
            Ok(Some(product)) => Ok(product),
            Ok(None) => Err(AppError::new("product not found", ErrorKind::NotFound)),
            Err(err) => {
                error!("Error fetching a product: {:?}", err);
                return Err(AppError::new(
                    "cannot fetch product",
                    ErrorKind::InternalServerError,
                ));
            }
        }
    }
}
