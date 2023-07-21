use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub fn establish_connection(client: &Client) -> Database {
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in env");

    // return a handle to the database
    client.database(&database_name)
}
