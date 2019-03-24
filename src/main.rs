extern crate gotham;
extern crate hyper;
extern crate mime;

use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;


pub fn index(state: State) -> (State, &'static str) {
    (state, "Hello World")
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(index);
    })
}

fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router());
    println!("Hello, world!");
}
