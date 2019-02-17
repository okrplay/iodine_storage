use sofa::{Client, Database};
use std::env;

pub fn establish_connection() -> Database {
    let client = Client::new(
        env::var("iodine_database_url").expect("Environment variable iodine_database_url not set"),
    )
    .expect("Failed to connect to database");
    client
        .db(env::var("iodine_database_name")
            .expect("Environment variable iodine_database_name not set"))
        .unwrap()
}
