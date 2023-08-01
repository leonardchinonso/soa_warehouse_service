use crate::{
    dto::product::product_dto::{
        AddProductRequest, AddProductResponse, CheckAvailabilityRequest, ClientId,
        ClientIdProductId, GetProductResponse, GetProductsResponse, ProcessOrderRequest,
        SetProductQuantityRequest, SetProductQuantityResponse, UpdateProductRequest,
        UpdateProductResponse,
    },
    dto::APIResponse,
    errors::app_error::{AppError, ErrorKind},
    model::product::{Product, ProductResponse},
    server,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Json, Path, Query},
    HttpResponse, Responder,
};
use log::error;
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;

// get_product is the handler to get a single product
#[get("/v1/{client_id}/products/{product_id}")]
pub async fn get_product(
    app_data: web::Data<server::AppState>,
    cp_id: Path<ClientIdProductId>,
) -> impl Responder {
    let client_id_product_id = cp_id.into_inner();

    // validate the client id
    let client_id = match ObjectId::from_str(&client_id_product_id.client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // validate the product id
    let product_id = match ObjectId::from_str(&&client_id_product_id.product_id.as_str()) {
        Ok(product_id) => product_id,
        Err(_) => {
            return AppError::new("invalid product id", ErrorKind::FailedAction).to_responder()
        }
    };

    // retrieve the product from the service
    let product = match app_data
        .service_manager
        .product_service
        .get_product(product_id, client_id)
        .await
    {
        Ok(product) => product,
        Err(err) => return err.to_responder(),
    };

    // return the product
    HttpResponse::Ok().json(APIResponse::success(
        "product retrieved successfully",
        GetProductResponse::new(product._id.to_hex(), product.name, product.description),
    ))
}

// get_product is the handler to get a single product
#[get("/v1/{client_id}/products")]
pub async fn get_products_by_client(
    app_data: web::Data<server::AppState>,
    c_id: Path<ClientId>,
) -> impl Responder {
    // try converting the client_id from string to an objectId
    let client_id = match ObjectId::from_str(c_id.into_inner().client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // retrieve the products from the service
    let products = match app_data
        .service_manager
        .product_service
        .get_products(client_id)
        .await
    {
        Ok(products) => products
            .iter()
            .map(|p| p.to_product_response())
            .collect::<Vec<ProductResponse>>(),
        Err(err) => return err.to_responder(),
    };

    // return the products
    HttpResponse::Ok().json(APIResponse::success(
        "products retrieved successfully",
        GetProductsResponse::new(products),
    ))
}

// add_product is the handler to add a product
#[post("/v1/{client_id}/products")]
pub async fn add_product(
    app_data: web::Data<server::AppState>,
    request: Json<AddProductRequest>,
    c_id: Path<ClientId>,
) -> impl Responder {
    // validate the request body
    if let Err(err) = request.validate() {
        return err.to_responder();
    }

    // try converting the client_id from string to an objectId
    let client_id = match ObjectId::from_str(c_id.into_inner().client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // build a new product object from the request
    let mut product = Product::new(request.name.clone(), request.description.clone(), client_id);

    // call the product service to handle creating the product
    if let Err(err) = app_data
        .service_manager
        .product_service
        .create(&mut product, client_id, request.quantity)
        .await
    {
        return err.to_responder();
    };

    // return the product
    HttpResponse::Ok().json(APIResponse::success(
        "product added successfully",
        AddProductResponse::new(
            product._id.to_hex(),
            product.name.clone(),
            product.description.clone(),
            product.get_sku(),
            request.quantity,
        ),
    ))
}

// update_product is the handler to update a product
#[put("/v1/{client_id}/products/{product_id}")]
pub async fn update_product(
    app_data: web::Data<server::AppState>,
    request: Json<UpdateProductRequest>,
    cp_id: Path<ClientIdProductId>,
) -> impl Responder {
    let client_id_product_id = cp_id.into_inner();

    // validate the client id
    let client_id = match ObjectId::from_str(&client_id_product_id.client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // validate the product id
    let product_id = match ObjectId::from_str(&&client_id_product_id.product_id.as_str()) {
        Ok(product_id) => product_id,
        Err(_) => {
            return AppError::new("invalid product id", ErrorKind::FailedAction).to_responder()
        }
    };

    // update the product using the service
    let product = match app_data
        .service_manager
        .product_service
        .update_product(client_id, product_id, &*request)
        .await
    {
        Ok(product) => product,
        Err(err) => return err.to_responder(),
    };

    // create and return the http response
    HttpResponse::Ok().json(APIResponse::success(
        "product updated successfully",
        UpdateProductResponse::new(
            product._id.to_hex(),
            product.name.clone(),
            product.description.clone(),
            product.get_sku(),
        ),
    ))
}

// set_product_quantity is the handler to set the quantity of a product
#[put("/v1/{client_id}/products/{product_id}/quantity")]
pub async fn set_product_quantity(
    app_data: web::Data<server::AppState>,
    request: Json<SetProductQuantityRequest>,
    cp_id: Path<ClientIdProductId>,
) -> impl Responder {
    let client_id_product_id = cp_id.into_inner();

    // validate the client id
    let client_id = match ObjectId::from_str(&client_id_product_id.client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // validate the product id
    let product_id = match ObjectId::from_str(&&client_id_product_id.product_id.as_str()) {
        Ok(product_id) => product_id,
        Err(_) => {
            return AppError::new("invalid product id", ErrorKind::FailedAction).to_responder()
        }
    };

    // update the product using the service
    let stock = match app_data
        .service_manager
        .product_service
        .set_product_quantity(client_id, product_id, &*request)
        .await
    {
        Ok(product) => product,
        Err(err) => return err.to_responder(),
    };

    HttpResponse::Ok().json(APIResponse::success(
        "product quantity set successfully",
        SetProductQuantityResponse::new(stock.get_quantity()),
    ))
}

// check_availability checks the quantity availability of products in a stock
#[get("/v1/{client_id}/products/{product_id}/availability")]
pub async fn check_availability(
    app_data: web::Data<server::AppState>,
    cp_id: Path<ClientIdProductId>,
    query: Query<CheckAvailabilityRequest>,
) -> impl Responder {
    let client_id_product_id = cp_id.into_inner();

    // validate the client id
    let client_id = match ObjectId::from_str(&client_id_product_id.client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // validate the product id
    let product_id = match ObjectId::from_str(&&client_id_product_id.product_id.as_str()) {
        Ok(product_id) => product_id,
        Err(_) => {
            return AppError::new("invalid product id", ErrorKind::FailedAction).to_responder()
        }
    };

    if let Err(err) = app_data
        .service_manager
        .product_service
        .check_availability(client_id, product_id, query.number)
        .await
    {
        return match err.kind {
            ErrorKind::FailedAction => err.to_responder(),
            ErrorKind::NotFound => err.to_responder(),
            _ => {
                error!("Error checking availability");
                AppError::new("cannot check availability", ErrorKind::InternalServerError)
                    .to_responder()
            }
        };
    }

    HttpResponse::Ok().json(APIResponse::success(
        "product is available in requested number",
        None::<String>,
    ))
}

// delete_product deletes a product and its stock from the application
#[delete("/v1/{client_id}/products/{product_id}")]
pub async fn delete_product(
    app_data: web::Data<server::AppState>,
    cp_id: Path<ClientIdProductId>,
) -> impl Responder {
    let client_id_product_id = cp_id.into_inner();

    // validate the client id
    let client_id = match ObjectId::from_str(&client_id_product_id.client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // validate the product id
    let product_id = match ObjectId::from_str(&&client_id_product_id.product_id.as_str()) {
        Ok(product_id) => product_id,
        Err(_) => {
            return AppError::new("invalid product id", ErrorKind::FailedAction).to_responder()
        }
    };

    // delete the product in the product service
    if let Err(err) = app_data
        .service_manager
        .product_service
        .delete_product(client_id, product_id)
        .await
    {
        return err.to_responder();
    }

    HttpResponse::Ok().json(APIResponse::success(
        "product deleted successfully",
        None::<String>,
    ))
}

// process_orders processes orders by decrementing their product quantity by the specified quantity
#[post("/v1/{client_id}/orders")]
pub async fn process_orders(
    request: Json<Vec<ProcessOrderRequest>>,
    c_id: Path<ClientId>,
    app_data: web::Data<server::AppState>,
) -> impl Responder {
    // validate the client id
    let client_id = match ObjectId::from_str(c_id.into_inner().client_id.as_str()) {
        Ok(client_id) => client_id,
        Err(_) => {
            return AppError::new("invalid client id", ErrorKind::FailedAction).to_responder()
        }
    };

    // process orders in the product service
    if let Err(err) = app_data
        .service_manager
        .product_service
        .process_orders(client_id, request.0)
        .await
    {
        return match err.kind {
            ErrorKind::FailedAction => err.to_responder(),
            ErrorKind::NotFound => err.to_responder(),
            _ => {
                error!("Error processing orders");
                AppError::new("cannot process orders", ErrorKind::InternalServerError)
                    .to_responder()
            }
        };
    }

    HttpResponse::Ok().json(APIResponse::success(
        "order processed successfully",
        None::<String>,
    ))
}
