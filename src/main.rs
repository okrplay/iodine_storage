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
mod database;
mod resources;

// imports
use dotenv::dotenv;
use tower_web::ServiceBuilder;

// main loop
fn main() {
    // load environment variables from .dotenv
    dotenv().ok();

    // set server address
    let addr = "127.0.0.1:7070".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    // construct the server and kick it off
    ServiceBuilder::new()
        .resource(resources::api::APIResource)
        .run(&addr)
        .unwrap();
}
