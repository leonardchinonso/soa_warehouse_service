use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use bson::Document;
use mongodb::{Database, Collection};

use crate::{api, model::{PRODUCT_COLLECTION, product::Product}, service::product_service::ProductService};

// ServiceManager is the struct for managing services
pub struct ServiceManager {
    pub product_service: ProductService,
}

// AppState holds the state of the application
pub struct AppState {
    pub service_manager: ServiceManager,
}

// implement service manager methods
impl ServiceManager {
    // start_services starts all the services and returns the manager for the services
    pub fn new(database: &Database) -> Self {
        // create the service worker for the product service
        let product_collection: Collection<Document> = database.collection(PRODUCT_COLLECTION);
        let product_service_worker = ProductService::new(product_collection);

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

        // initialize cors for the resource gatekeeping
        let cors_middleware = Cors::default()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        // launch the http server
        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(api::init)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
