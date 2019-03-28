use super::entity_header::*;

#[derive(Serialize, Copy, Clone)]
pub struct User {
    pub id: &'static str,
    pub username: &'static str,
    pub email: &'static str,
    pub fullname: &'static str,
    pub password: &'static str
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

#[derive(Serialize, Clone)]
pub struct UserList {
    pub data: Vec<User>,
    pub length: usize
}

impl UserList {
    pub fn new(user_list: Vec<User>) -> UserList {
        UserList {
            data: user_list.clone(),
            length: user_list.len()
        }
    }
}

impl IntoResponse for UserList {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized user list")
        )
    }
}

