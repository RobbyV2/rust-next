use axum::response::Json;
use serde_json::json;

use super::ApiResponse;

#[utoipa::path(get, path = "/api/hello", responses((status = 200, body = ApiResponse)))]
pub(crate) async fn handler() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello from Rust API!".into(),
        data: Some(json!({ "status": "success", "version": env!("CARGO_PKG_VERSION") })),
    })
}
