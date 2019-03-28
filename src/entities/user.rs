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

