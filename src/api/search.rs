use axum::{extract::Query, http::StatusCode, response::Json};
use serde::Deserialize;
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

use super::ApiResponse;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub(crate) struct Params {
    q: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/search",
    params(Params),
    responses(
        (status = 200, body = ApiResponse),
        (status = 400, description = "Missing or empty query parameter")
    )
)]
pub(crate) async fn handler(
    Query(Params { q }): Query<Params>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let query = q.filter(|s| !s.is_empty()).ok_or(StatusCode::BAD_REQUEST)?;
    Ok(Json(ApiResponse {
        message: format!("Search results for: {query}"),
        data: Some(json!({ "query": query, "results": [] })),
    }))
}
