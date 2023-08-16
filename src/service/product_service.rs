use bson::oid::ObjectId;
use futures::future;
use log::error;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::{
    dto::product::product_dto::{
        ProductQuantity, ProductQuantityRequest, SetProductQuantityRequest, UpdateProductRequest,
    },
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
    // new creates a new product service instance
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

        // insert the client_id, product_id and quantity in stock
        let stock = Stock::new(client_id, product_id, quantity);
        if let Err(err) = self.stock_repo.insert(&stock).await {
            error!("Error creating stock: {:?}", err);
            return Err(AppError::new(
                "cannot create product",
                ErrorKind::InternalServerError,
            ));
        }

        Ok(())
    }

    // get_product gets a product and its quantity from the application storage
    pub async fn get_product(
        &self,
        product_id: ObjectId,
        client_id: ObjectId,
    ) -> Result<(Product, i32), AppError> {
        let product = match self.product_repo.get_by_id(client_id, product_id).await {
            Ok(Some(product)) => product,
            Ok(None) => return Err(AppError::new("product not found", ErrorKind::NotFound)),
            Err(err) => {
                error!("Error fetching a product: {:?}", err);
                return Err(AppError::new(
                    "cannot fetch product",
                    ErrorKind::InternalServerError,
                ))
            }
        };

        let stock = match self.stock_repo.get_by_client_id_and_product_id(client_id, product_id).await {
            Ok(Some(stock)) => stock,
            Ok(None) => return Err(AppError::new("stock not found", ErrorKind::NotFound)),
            Err(err) => {
                error!("Error fetching a stock: {:?}", err);
                return Err(AppError::new(
                    "cannot fetch stock",
                    ErrorKind::InternalServerError,
                ))
            }
        };

        Ok((product, stock.get_quantity()))
    }

    // get_products gets products from the application storage
    pub async fn get_products_by_client(&self, client_id: ObjectId) -> Result<Vec<(Product, i32)>, AppError> {
        let mut pq_hashmap: HashMap<ObjectId, (Product, i32)> = HashMap::new();

        let products = match self.product_repo.get_by_client_id(client_id).await {
            Ok(products) => products,
            Err(err) => {
                error!("Error fetching all products: {:?}", err);
                return Err(AppError::new(
                    "cannot fetch products",
                    ErrorKind::InternalServerError,
                ));
            }
        };

        // insert the products in the hashmap
        for product in products {
            pq_hashmap.insert(product._id, (product, 0));
        }

        let stocks = match self.stock_repo.get_by_client_id(client_id).await {
            Ok(stocks) => stocks,
            Err(err) => {
                error!("Error fetching all stocks: {:?}", err);
                return Err(AppError::new(
                    "cannot fetch stocks",
                    ErrorKind::InternalServerError,
                ));
            }
        };

        for stock in stocks {
            pq_hashmap
                .get_mut(&stock.product_id)
                .map(|pq| pq.1 = stock.get_quantity());
        }

        let mut pq_vec: Vec<(Product, i32)> = Vec::with_capacity(pq_hashmap.len());

        for (_, pq) in pq_hashmap {
            pq_vec.push(pq);
        }

        Ok(pq_vec)
    }

    // update_product updates a product in the application storage
    pub async fn update_product(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
        update: &UpdateProductRequest,
    ) -> Result<Product, AppError> {
        // retrieve the product from the service
        let mut product = self.get_product(product_id, client_id).await?.0;

        // update the fields in the product
        product.name = update.name.clone();
        product.description = update.description.clone();

        // get the result of updating the document
        let result = self.product_repo.update(client_id, &product).await;

        // if failed, stop process flow and log error.
        if result.is_err() {
            error!(
                "Error updating product with id: {:?}. Error: {:?}",
                product_id,
                result.unwrap_err()
            );
            return Err(AppError::new(
                "cannot update product",
                ErrorKind::InternalServerError,
            ));
        }

        // if nothing was modified, return an error
        if result.unwrap().modified_count == 0 {
            error!(
                "Error updating product with id: {:?}. No documents were modified",
                product_id
            );
            return Err(AppError::new(
                "cannot update product",
                ErrorKind::InternalServerError,
            ));
        }

        Ok(product)
    }

    // set_product_quantity sets the quantity of a product in stock to
    // to a value higher than it previously was. If the value is less than
    // the current quantity, an error is returned.
    pub async fn set_product_quantity(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
        update: &SetProductQuantityRequest,
    ) -> Result<Stock, AppError> {
        // check that the product exists and get the stock if it does
        let mut stock = self
            .check_product_and_get_stock(client_id, product_id)
            .await?;

        // compare the current quantity with the quantity to set, must be lower
        if update.quantity < stock.get_quantity() {
            return Err(AppError::new(
                "quantity to set must be more than current quantity",
                ErrorKind::FailedAction,
            ));
        }

        // if the quantities are equal, no need to update
        if update.quantity == stock.get_quantity() {
            return Ok(stock);
        }

        // set the quantity in the stock object
        stock.set_quantity(update.quantity);
        // update the quantity in the stock
        let modified_count = match self.stock_repo.update(&stock).await {
            Ok(result) => result.modified_count,
            Err(_) => {
                return Err(AppError::new(
                    "cannot set quantity",
                    ErrorKind::InternalServerError,
                ))
            }
        };
        if modified_count == 0 {
            return Err(AppError::new(
                "cannot set quantity",
                ErrorKind::InternalServerError,
            ));
        }

        Ok(stock)
    }

    // check_availability checks if a product has the required number in stock
    pub async fn check_availability(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
        has_available: i32,
    ) -> Result<(), AppError> {
        // check that the product exists and get the stock if it does
        let stock = self
            .check_product_and_get_stock(client_id, product_id)
            .await?;

        // compare the current quantity with the requested quantity
        if has_available > stock.get_quantity() {
            return Err(AppError::new(
                "product quantity is less than requested number",
                ErrorKind::FailedAction,
            ));
        }

        Ok(())
    }

    // check_multiple_availability checks multiple products have their required number in the stock
    pub async fn check_multiple_availability(
        &self,
        client_id: ObjectId,
        check_requests: Vec<ProductQuantityRequest>,
    ) -> Result<(), AppError> {
        // create a vector of checks to process
        let pq_vec = match self.check_products_and_convert_to_product_quantity_vector(
            check_requests
        ).await {
            Ok(orders) => orders,
            Err(err) => return Err(err),
        };

        // if any product is not available in quantity, return an error
        if let Err(err) = self.check_availability_many(client_id, &pq_vec).await {
            return Err(err);
        }

        Ok(())
    }

    // delete_product deletes a product and its stock from the database
    pub async fn delete_product(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
    ) -> Result<(), AppError> {
        // delete the product from the database
        if let Err(err) = self.product_repo.delete_by_id(client_id, product_id).await {
            error!(
                "Error deleting product with id: {:?} and client_id: {:?}. Error: {:?}",
                product_id, client_id, err
            );
            return Err(AppError::new(
                "cannot delete product",
                ErrorKind::InternalServerError,
            ));
        }

        // delete the stock from the database
        if let Err(err) = self
            .stock_repo
            .delete_by_client_id_and_product_id(client_id, product_id)
            .await
        {
            error!(
                "Error deleting stock with client_id: {:?} and product_id: {:?}. Error: {:?}",
                client_id, product_id, err
            );
            return Err(AppError::new(
                "cannot delete stock",
                ErrorKind::InternalServerError,
            ));
        };

        Ok(())
    }

    // process_orders checks that all orders are eligible to be processes then processes them
    pub async fn process_orders(
        &self,
        client_id: ObjectId,
        order_requests: Vec<ProductQuantityRequest>,
    ) -> Result<(), AppError> {
        // create a vector of orders to process
        let orders = match self.check_products_and_convert_to_product_quantity_vector(
            order_requests
        ).await {
            Ok(orders) => orders,
            Err(err) => return Err(err),
        };

        // if any product is not available in quantity, return an error
        if let Err(err) = self.check_availability_many(client_id, &orders).await {
            return Err(err);
        }

        // create an empty list of unresolved futures for decrementing the quantity of products
        let mut decrement_quantity_futs = Vec::with_capacity(orders.len());
        // go ahead to decrement their counts since they're all available
        for order in &orders {
            // push the asynchronous call to the list of unresolved futures
            decrement_quantity_futs.push(self.decrement_quantity_by(
                client_id,
                order.product_id,
                order.quantity,
            ));
        }
        // ensure the decrements ran without error
        if let Err(err) = future::try_join_all(decrement_quantity_futs).await {
            return Err(err);
        }

        Ok(())
    }

    // check_product_and_get_stock checks that a product exists and retrieves the stock
    async fn check_product_and_get_stock(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
    ) -> Result<Stock, AppError> {
        // check that the product exists
        if let Err(err) = self.get_product(product_id, client_id).await {
            return Err(err);
        }

        // retrieve the stock for the product
        match self
            .stock_repo
            .get_by_client_id_and_product_id(client_id, product_id)
            .await
        {
            Ok(Some(stock)) => Ok(stock),
            Ok(None) => return Err(AppError::new("stock not found", ErrorKind::NotFound)),
            Err(_) => {
                error!(
                    "Error getting stock from client_id: {:?}, and product_id: {:?}",
                    client_id, product_id
                );
                return Err(AppError::new(
                    "cannot get product quantity",
                    ErrorKind::InternalServerError,
                ));
            }
        }
    }

    async fn check_products_and_convert_to_product_quantity_vector(
        &self,
        pq_requests: Vec<ProductQuantityRequest>,
    ) -> Result<Vec<ProductQuantity>, AppError> {
        // create a vector of checks to process
        let mut pq_vec: Vec<ProductQuantity> = Vec::with_capacity(pq_requests.len());

        let mut seen_checks = HashSet::new();

        // convert all product ids to objectIds and check for duplicity
        for pq_request in pq_requests {
            let mut pq = ProductQuantity::new();
            pq.product_id = match ObjectId::from_str(pq_request.product_id.as_str()) {
                Ok(product_id) => product_id,
                Err(_) => {
                    return Err(AppError::new(
                        &format!("invalid product id: {:?}", pq_request.product_id),
                        ErrorKind::FailedAction,
                    ))
                }
            };

            // check if the order has been seen before
            if seen_checks.contains(&pq.product_id) {
                return Err(AppError::new(
                    &format!("duplicate order with product_id: {:?}", pq.product_id),
                    ErrorKind::FailedAction,
                ));
            }

            // set the quantity of the order
            pq.quantity = pq_request.quantity;
            // add the order's product_id to the set
            seen_checks.insert(pq.product_id.clone());
            // add the created order to the list of orders
            pq_vec.push(pq);
        }

        Ok(pq_vec)
    }

    async fn check_availability_many(&self, client_id: ObjectId, checks: &Vec<ProductQuantity>) -> Result<(), AppError> {
        // create an empty list of unresolved futures for checking the availability of all orders
        let mut check_availability_futs = Vec::with_capacity(checks.len());
        // verify all products in the list have quantities requested, return error if a product is unavailable
        for check in checks {
            // push the asynchronous call to the list of unresolved futures
            check_availability_futs.push(self.check_availability(
                client_id,
                check.product_id,
                check.quantity,
            ));
        }
        // ensure the products in the orders are all available
        if let Err(err) = future::try_join_all(check_availability_futs).await {
            return Err(err);
        }

        Ok(())
    }

    // decrement_quantity_by decrements the quantity of a product in the stock by the given number
    // should only be called when it is ensured that the product exists
    async fn decrement_quantity_by(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
        number: i32,
    ) -> Result<(), AppError> {
        let mut stock = match self
            .stock_repo
            .get_by_client_id_and_product_id(client_id, product_id)
            .await
        {
            Ok(Some(stock)) => stock,
            Ok(None) => return Err(AppError::new("stock not found", ErrorKind::NotFound)),
            Err(_) => {
                error!(
                    "Error getting stock from client_id: {:?}, and product_id: {:?}",
                    client_id, product_id
                );
                return Err(AppError::new(
                    "cannot get product quantity",
                    ErrorKind::InternalServerError,
                ));
            }
        };

        stock.set_quantity(stock.get_quantity() - number);
        if stock.get_quantity() < 0 {
            return Err(AppError::new(
                "product is low in stock",
                ErrorKind::FailedAction,
            ));
        }

        let modified_count = match self.stock_repo.update(&stock).await {
            Ok(result) => result.modified_count,
            Err(_) => {
                return Err(AppError::new(
                    "cannot set quantity",
                    ErrorKind::InternalServerError,
                ))
            }
        };
        if modified_count == 0 {
            // if no document was modified, return an error
            return Err(AppError::new(
                "cannot decrement quantity",
                ErrorKind::InternalServerError,
            ));
        }

        Ok(())
    }
}
