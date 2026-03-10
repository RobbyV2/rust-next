use axum::{Router, extract::Query, http::StatusCode, response::Json, routing::get};
use serde::Deserialize;
use serde_json::json;

use super::ApiResponse;

#[derive(Debug, Deserialize)]
struct Params {
    q: Option<String>,
}

pub fn routes() -> Router {
    Router::new().route("/search", get(handler))
}

async fn handler(Query(Params { q }): Query<Params>) -> Result<Json<ApiResponse>, StatusCode> {
    let query = q.filter(|s| !s.is_empty()).ok_or(StatusCode::BAD_REQUEST)?;
    Ok(Json(ApiResponse {
        message: format!("Search results for: {query}"),
        data: Some(json!({ "query": query, "results": [] })),
    }))
}
