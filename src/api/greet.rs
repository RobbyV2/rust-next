use axum::{extract::Path, response::Json};

use super::ApiResponse;

#[utoipa::path(
    get,
    path = "/api/greet/{name}",
    params(("name" = String, Path, description = "Name to greet")),
    responses((status = 200, body = ApiResponse))
)]
pub(crate) async fn handler(Path(name): Path<String>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("Hello, {name}!"),
        data: None,
    })
}
