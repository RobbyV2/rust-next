use axum::{Router, http::StatusCode, response::Json, routing::post};
use serde::Deserialize;
use serde_json::json;

use super::ApiResponse;

#[derive(Debug, Deserialize)]
struct Payload {
    name: String,
}

pub fn routes() -> Router {
    Router::new().route("/create", post(handler))
}

async fn handler(Json(Payload { name }): Json<Payload>) -> Result<Json<ApiResponse>, StatusCode> {
    if name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(ApiResponse {
        message: format!("Created: {name}"),
        data: Some(json!({ "id": 1, "name": name })),
    }))
}
