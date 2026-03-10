mod create;
mod env;
mod greet;
mod hello;
mod search;

use axum::Router;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub fn routes() -> Router {
    Router::new()
        .merge(hello::routes())
        .merge(greet::routes())
        .merge(search::routes())
        .merge(create::routes())
        .merge(env::routes())
}
