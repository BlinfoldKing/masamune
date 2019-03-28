extern crate gotham;

use super::entities::user::User;

use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;

mod user;
use user::UserHandler;
use user::UserParams;

pub fn generate_routes() -> Router {
    build_simple_router(|router| {
        router
            .get("/user/:id")
            .with_path_extractor::<UserParams>()
            .to(UserHandler::get);
    })
}

