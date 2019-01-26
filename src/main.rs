#[macro_use]
extern crate tower_web;

mod resources;

use tower_web::ServiceBuilder;

fn main() {
    let addr = "127.0.0.1:7070".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(resources::api::APIResource)
        .run(&addr)
        .unwrap();
}
