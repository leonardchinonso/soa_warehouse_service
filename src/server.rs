use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use mongodb::Database;

use crate::{
    api,
    model::{product::Product, stock::Stock, PRODUCT_COLLECTION, STOCK_COLLECTION},
    repository::{product_repo::ProductRepo, stock_repo::StockRepo},
    service::product_service::ProductService,
};

// ServiceManager is the struct for managing services
pub struct ServiceManager {
    pub product_service: ProductService,
}

// AppState holds the state of the application
pub struct AppState {
    pub service_manager: ServiceManager,
}

// contains methods for managing the application state
impl AppState {
    pub fn new(service_manager: ServiceManager) -> Self {
        Self { service_manager }
    }
}

// implement service manager methods
impl ServiceManager {
    // start_services starts all the services and returns the manager for the services
    pub fn new(database: &Database) -> Self {
        // create the injections for the product service worker
        let product_collection = database.collection::<Product>(PRODUCT_COLLECTION);
        let stock_collection = database.collection::<Stock>(STOCK_COLLECTION);
        let product_repo_worker = ProductRepo::new(product_collection);
        let stock_repo_worker = StockRepo::new(stock_collection);
        let product_service_worker = ProductService::new(product_repo_worker, stock_repo_worker);

        // build and return the service manager
        ServiceManager {
            product_service: product_service_worker,
        }
    }
}

// start_server starts and launches the http server
pub async fn start_server(database: Database) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        // get the handle for the service manager
        let service_manager = ServiceManager::new(&database);

        // initialize cors for the resource gate keeping
        let _cors_middleware = Cors::default()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        // launch the http server
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState::new(service_manager)))
            .configure(api::init)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
