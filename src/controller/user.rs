extern crate serde_derive;

use super::State;
use super::User;
use super::FromState;

pub struct UserHandler;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct UserParams {
    id: String
}

impl UserHandler {
    pub fn get(state: State) -> (State, Result<User, String>) {
        let params = UserParams::borrow_from(&state);
        let users = generate_mock_user(10);

        for user in users.into_iter() {
            if user.id == params.id {
                return (state, Ok(user));
            }
        }

        (state, Err("Hello World".to_string()))
    }
}

fn generate_mock_user(n: u8) -> Vec<User> {
    let mut result = Vec::new();
    for i in 0..n {
        result.push(User {
            username: "blinfoldking",
            email: "blinf@oldking",
            fullname: "blinf oldking",
            id: "123",
            password: "wkwkwkwk"
        })
    }

    return result.clone()
}
