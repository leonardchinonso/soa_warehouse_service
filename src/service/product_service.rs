use std::collections::HashSet;
use bson::oid::ObjectId;
use futures::future;
use log::error;
use std::str::FromStr;

use crate::{
    dto::product::product_dto::{
        Order, ProcessOrderRequest, SetProductQuantityRequest, UpdateProductRequest,
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

    // get_product gets a product from the application storage
    pub async fn get_product(
        &self,
        product_id: ObjectId,
        client_id: ObjectId,
    ) -> Result<Product, AppError> {
        match self.product_repo.get_by_id(product_id, client_id).await {
            Ok(Some(product)) => Ok(product),
            Ok(None) => Err(AppError::new("product not found", ErrorKind::NotFound)),
            Err(err) => {
                error!("Error fetching a product: {:?}", err);
                Err(AppError::new(
                    "cannot fetch product",
                    ErrorKind::InternalServerError,
                ))
            }
        }
    }

    // get_products gets products from the application storage
    pub async fn get_products(&self, client_id: ObjectId) -> Result<Vec<Product>, AppError> {
        match self.product_repo.get_by_client_id(client_id).await {
            Ok(products) => Ok(products),
            Err(err) => {
                error!("Error fetching all products: {:?}", err);
                return Err(AppError::new(
                    "cannot fetch products",
                    ErrorKind::InternalServerError,
                ));
            }
        }
    }

    // update_product updates a product in the application storage
    pub async fn update_product(
        &self,
        client_id: ObjectId,
        product_id: ObjectId,
        update: &UpdateProductRequest,
    ) -> Result<Product, AppError> {
        // retrieve the product from the service
        let mut product = self.get_product(product_id, client_id).await?;

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
        order_requests: Vec<ProcessOrderRequest>,
    ) -> Result<(), AppError> {
        // create a vector of orders to process
        let mut orders: Vec<Order> = Vec::with_capacity(order_requests.len());

        let mut seen_orders = HashSet::new();

        // convert all product ids to objectIds and check for duplicity
        for order_request in order_requests {
            let mut order = Order::new();
            order.product_id = match ObjectId::from_str(order_request.product_id.as_str()) {
                Ok(product_id) => product_id,
                Err(_) => {
                    return Err(AppError::new(
                        &format!("invalid product id: {:?}", order_request.product_id),
                        ErrorKind::FailedAction,
                    ))
                }
            };

            // check if the order has been seen before
            if seen_orders.contains(&order.product_id) {
                return Err(AppError::new(&format!("duplicate order with product_id: {:?}", order.product_id), ErrorKind::FailedAction))
            }

            // set the quantity of the order
            order.quantity = order_request.quantity;
            // add the order's product_id to the set
            seen_orders.insert(order.product_id.clone());
            // add the created order to the list of orders
            orders.push(order);
        }

        // create an empty list of unresolved futures for checking the availability of all orders
        let mut check_availability_futs = Vec::with_capacity(orders.len());
        // verify all products in the list have quantities requested, return error if a product is unavailable
        for order in &orders {
            // push the asynchronous call to the list of unresolved futures
            check_availability_futs.push(self.check_availability(
                client_id,
                order.product_id,
                order.quantity,
            ));
        }
        // ensure the products in the orders are all available
        if let Err(err) = future::try_join_all(check_availability_futs).await {
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
