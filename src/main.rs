extern crate gotham;
extern crate hyper;
extern crate mime;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::{ Body, Response, StatusCode };

use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;
use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;

#[derive(Serialize)]
pub struct User {
    username: String
}

impl IntoResponse for User {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized user")
        )
    }
}

pub fn index(state: State) -> (State, User) {
    (state, User { username: "blinfoldking".to_string() })
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

