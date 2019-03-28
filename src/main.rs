extern crate gotham;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate gotham_derive;

pub mod entities;
mod controller;

fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, controller::generate_routes());
    println!("Hello, world!");
}

