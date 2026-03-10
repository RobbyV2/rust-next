use axum::Router;

pub mod route_builder;

pub fn build_router(proxy_url: Option<&str>) -> Router {
    route_builder::register_routes(proxy_url)
}
