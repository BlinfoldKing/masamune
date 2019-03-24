use super::entity_header::*;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub fullname: String,
    pub password: String
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

