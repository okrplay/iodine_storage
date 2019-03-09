// imports
use sofa::{Client, Database};
use std::env;

// function to establish couchdb connection and get a database
pub fn establish_connection() -> Database {
    // create the couchdb client
    let client = Client::new(
        env::var("iodine_database_url").expect("Environment variable iodine_database_url not set"),
    )
    .expect("Failed to connect to database");
    // get the database
    client
        .db(env::var("iodine_database_name")
            .expect("Environment variable iodine_database_name not set"))
        .unwrap()
}
