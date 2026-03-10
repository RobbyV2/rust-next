use axum::{Router, response::Json, routing::get};
use serde_json::json;

use super::ApiResponse;

pub fn routes() -> Router {
    Router::new().route("/hello", get(handler))
}

async fn handler() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello from Rust API!".into(),
        data: Some(json!({ "status": "success", "version": env!("CARGO_PKG_VERSION") })),
    })
}
