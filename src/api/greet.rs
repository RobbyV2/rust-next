use axum::{Router, extract::Path, response::Json, routing::get};

use super::ApiResponse;

pub fn routes() -> Router {
    Router::new().route("/greet/{name}", get(handler))
}

async fn handler(Path(name): Path<String>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("Hello, {name}!"),
        data: None,
    })
}
