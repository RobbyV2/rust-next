use axum::{http::StatusCode, response::Json};
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;

use super::ApiResponse;

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct Payload {
    name: String,
}

#[utoipa::path(
    post,
    path = "/api/create",
    request_body = Payload,
    responses(
        (status = 200, body = ApiResponse),
        (status = 400, description = "Empty name")
    )
)]
pub(crate) async fn handler(
    Json(Payload { name }): Json<Payload>,
) -> Result<Json<ApiResponse>, StatusCode> {
    if name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(ApiResponse {
        message: format!("Created: {name}"),
        data: Some(json!({ "id": 1, "name": name })),
    }))
}
