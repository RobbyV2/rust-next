pub(crate) mod create;
pub(crate) mod env;
pub(crate) mod greet;
pub(crate) mod hello;
pub mod openapi;
pub(crate) mod search;

use axum::{
    Router,
    routing::{get, post},
};
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello::handler))
        .route("/greet/{name}", get(greet::handler))
        .route("/search", get(search::handler))
        .route("/create", post(create::handler))
        .route("/env", get(env::handler))
        .merge(openapi::routes())
}
