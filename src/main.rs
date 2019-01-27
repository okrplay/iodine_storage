extern crate dotenv;
extern crate frank_jwt;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate tower_web;

mod database;
mod resources;

use dotenv::dotenv;
use tower_web::ServiceBuilder;

fn main() {
    dotenv().ok();

    let addr = "127.0.0.1:7070".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(resources::api::APIResource)
        .run(&addr)
        .unwrap();
}
