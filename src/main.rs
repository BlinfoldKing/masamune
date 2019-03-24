extern crate gotham;
#[macro_use]
extern crate serde_derive;

use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;

mod entities;
use entities::user::User;

pub fn index(state: State) -> (State, User) {
    (
        state,
        User {
            username: "blinfoldking".to_string(),
            email: "blinf@oldking".to_string(),
            fullname: "blinf oldking".to_string(),
            id: "123".to_string(),
            password: "wkwkwkwk".to_string()
        }
    )
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

