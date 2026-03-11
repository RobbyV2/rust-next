use axum::Router;

use crate::config::AppConfig;

pub mod route_builder;

pub fn build_router(proxy_url: Option<&str>, config: &AppConfig) -> Router {
    route_builder::register_routes(proxy_url, config)
}
