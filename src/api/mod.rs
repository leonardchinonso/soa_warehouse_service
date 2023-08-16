use actix_web::web;

pub mod product_router;

// init configures routes for the application
pub fn init(cfg: &mut web::ServiceConfig) {
    // product services
    cfg.service(product_router::add_product);
    cfg.service(product_router::get_product);
    cfg.service(product_router::get_products_by_client);
    cfg.service(product_router::update_product);
    cfg.service(product_router::set_product_quantity);
    cfg.service(product_router::check_availability);
    cfg.service(product_router::check_multiple_availability);
    cfg.service(product_router::delete_product);
    cfg.service(product_router::process_orders);
}
