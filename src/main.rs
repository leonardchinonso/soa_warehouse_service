mod api;
mod errors;
mod model;
mod repository;
mod server;
mod service;
mod utils;
mod dto;

use api::product_router::get_product;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use log::info;
use mongodb::{options::ClientOptions, Client};
use repository::mongo::establish_connection;
use server::start_server;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // initialize the environment variable reader
    dotenv().ok();

    // set up env variables for middleware logging
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    // initialize the logger
    env_logger::init();

    // get the database url from the env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");
    // get a handle on the client connection using the client options to build it
    let client_opts = ClientOptions::parse(&database_url)
        .await
        .expect("DATABASE_URL is incorrect");
    let client =
        Client::with_options(client_opts).expect("failed to start client with client_options");

    // establish connection to the database and get the handler
    let db = establish_connection(&client);

    info!("Connected to database successfully!");

    // start the server
    start_server(db.clone()).await
}
