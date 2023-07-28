use actix_web::web;

pub mod product_router;

// init configures routes for the application
pub fn init(cfg: &mut web::ServiceConfig) {
    // product services
    cfg.service(product_router::add_product);
}
