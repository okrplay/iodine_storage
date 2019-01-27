use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("database_url").expect("Environment variable database_url not set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
