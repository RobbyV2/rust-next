use axum::{Router, response::Json, routing::get};
use serde_json::json;

use super::ApiResponse;

pub fn routes() -> Router {
    Router::new().route("/env", get(handler))
}

fn env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| "not set".into())
}

async fn handler() -> Json<ApiResponse> {
    let secret = env("SECRET_KEY");
    let secret_status = match secret.as_str() {
        "not set" => "Not set".into(),
        s => format!("Set ({}...)", &s[..s.len().min(3)]),
    };

    Json(ApiResponse {
        message: "Environment variables from Rust".into(),
        data: Some(json!({
            "APP_MODE": env("APP_MODE"),
            "SERVER_HOST": env("SERVER_HOST"),
            "SERVER_PORT": env("SERVER_PORT"),
            "RUST_LOG": env("RUST_LOG"),
            "SECRET_KEY": secret_status,
        })),
    })
}
