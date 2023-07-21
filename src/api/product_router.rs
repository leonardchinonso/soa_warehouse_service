use log::{error, info};
use crate::{
    dto::product::{
        product_error::ProductError,
        product_dto::AddProductRequest,
    },
    model::{
        product::{self, Product},
    },
    service,
    server,
};
use actix_web::{
    get, post,
    http::{header::ContentType, StatusCode},
    web::{self, Json, Path, Data},
    HttpResponse, ResponseError, Responder,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
// struct to aid extractor in extracting the product id
pub struct ProductId {
    product_id: u32,
}

// endpoint to get a single product
#[get("/v1/products/{product_id}")]
pub async fn get_product(p_id: Path<ProductId>) -> Result<Json<Product>, ProductError> {
    info!("GET PRODUCT!!!");
    // get the product_id from the path by casting it to a json i64
    // let product_id = Json(p_id.into_inner().product_id);

    // get the product from the service
    // let prods = service::product_service::get_product(*product_id)?;

    // match prods.first() {
    //     Some(first) => Ok(Json(first.clone())),
    //     None => Err(ProductError::ProductNotFound)
    // }

    todo!()
}

#[post("/v1/products/")]
pub async fn add_product(app_data: web::Data<server::AppState>, request: Json<AddProductRequest>) -> impl Responder {
    info!("POST PRODUCT!!!");

    // build a new product object from the request
    let product = Product::new(request.name.clone(), request.description.clone());

    // call the product service to handle the getting of product
    let result = app_data.service_manager.product_service.create(&product).await;

    // let result = web::block(move || action).await;

    match result {
        Ok(insert_one_result) => {
            if let Some(oid) = insert_one_result.inserted_id.as_object_id() {
                HttpResponse::Ok().json(oid.to_hex())
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(e) => {
            error!("Error while creating a product: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}