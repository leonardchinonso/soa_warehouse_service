use crate::{
    dto::product::product_dto::{AddProductRequest, AddProductResponse, ClientId, ProductId},
    errors::app_error::{AppError, ErrorKind},
    model::product::Product,
    server,
};
use actix_web::{
    get, post,
    web::{self, Json, Path},
    HttpResponse, Responder,
};
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;

// get_product is the handler to get a single product
#[get("/v1/products/{product_id}")]
pub async fn get_product(p_id: Path<ProductId>) -> impl Responder {
    return AppError::new("UNIMPLEMENTED", ErrorKind::InternalServerError).to_responder();
}

// add_product is the handler to add a product
#[post("/v1/products/{client_id}")]
pub async fn add_product(
    request: Json<AddProductRequest>,
    c_id: Path<ClientId>,
    app_data: web::Data<server::AppState>,
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
    let mut product = Product::new(request.name.clone(), request.description.clone());

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
    let response = AddProductResponse::new(
        product._id.to_hex(),
        product.name,
        product.description,
        request.quantity,
    );
    HttpResponse::Ok().json(response)
}
