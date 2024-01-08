use axum::{
    http::{Response, StatusCode},
    Json,
};
use axum_core::{body::Body, response::IntoResponse};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct JsonResponse {
    #[serde(serialize_with = "serialize_status_code")]
    status_code: StatusCode,
    message: String,
}

impl JsonResponse {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Self {
            status_code,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response<Body> {
        // Serialize the response to a pretty JSON string
        let pretty_json_body = serde_json::to_string_pretty(&self).unwrap(); // Handle error appropriately
        (self.status_code, pretty_json_body).into_response()
    }
}

fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}