mod api;
mod dto;
mod errors;
mod model;
mod repository;
mod server;
mod service;
mod utils;

use dotenv::dotenv;
use log::info;
use mongodb::{options::ClientOptions, Client};
use repository::mongo;
use server::start_server;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // initialize the environment variable reader
    dotenv().ok();

    // set up env variables for middleware logging
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,debug");
    env::set_var("RUST_BACKTRACE", "1");

    // initialize the logger
    env_logger::init();

    // get the environment currently working on
    let build_env = env::var("BUILD_ENVIRONMENT").expect("BUILD_ENVIRONMENT must be set in env");

    // get the database environment based on the selected building environment
    let db_url_env_name = match build_env.as_str() {
        "development" => String::from("LOCAL_DATABASE_URL"),
        "staging" => String::from("ATLAS_DATABASE_URL"),
        _ => panic!("invalid environment"),
    };

    // get the database url from the env
    let database_url = env::var(db_url_env_name.clone()).expect(format!("{} must be set in env", db_url_env_name).as_str());

    // get a handle on the client connection using the client options to build it
    let client_opts = ClientOptions::parse(&database_url)
        .await
        .expect("DATABASE_URL is incorrect");
    let client =
        Client::with_options(client_opts).expect("failed to start client with client_options");

    // establish connection to the database and get the handler
    let db = mongo::establish_connection(&client);

    info!("Connected to database successfully!");

    // start the server
    start_server(db.clone()).await
}
