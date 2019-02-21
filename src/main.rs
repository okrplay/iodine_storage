// import crates
extern crate dotenv;
extern crate frank_jwt;
extern crate http;
extern crate serde;
extern crate sofa;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate tower_web;

// import modules
mod authentication;
mod database;
mod resources;
mod responses;

// imports
use database::{connection::establish_connection, setup::check_setup};
use dotenv::dotenv;
use std::env;
use tower_web::ServiceBuilder;

// main loop
fn main() {
    // load environment variables from .dotenv
    dotenv().ok();

    // set server address
    let addr = env::var("iodine_url")
        .expect("Environment variable iodine_url not set")
        .parse()
        .expect("Invalid address");
    println!("Listening on http://{}", addr);

    // check for existing setup
    let conn = establish_connection();
    if check_setup(conn) {
        println!("Iodine is already setup, starting.");
    } else {
        println!("Iodine is not setup, please visit {}/setup.", addr)
    }

    // construct the server and kick it off
    ServiceBuilder::new()
        .resource(resources::api::APIResource)
        .run(&addr)
        .unwrap();
}
