use std::sync::Arc;

use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

pub fn register_routes(proxy_url: Option<&str>) -> Router {
    let api_routes = crate::api::routes();
    let router = Router::new().nest("/api", api_routes);

    match proxy_url {
        Some(url) => {
            let url: Arc<str> = Arc::from(url);
            router.fallback(move |req| {
                let url = Arc::clone(&url);
                async move { proxy_to_frontend(url, req).await }
            })
        }
        None => router,
    }
}

async fn proxy_to_frontend(proxy_url: Arc<str>, mut req: Request) -> Response {
    let path_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or(req.uri().path());

    let new_uri = format!("{proxy_url}{path_query}");
    match new_uri.parse() {
        Ok(uri) => *req.uri_mut() = uri,
        Err(e) => {
            tracing::error!("Failed to parse URI {new_uri}: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI").into_response();
        }
    }

    if let Ok(proxy_uri) = proxy_url.parse::<hyper::Uri>()
        && let Some(host) = proxy_uri.host()
    {
        let host_val = match proxy_uri.port_u16() {
            Some(port) => format!("{host}:{port}"),
            None => host.to_string(),
        };
        if let Ok(hv) = host_val.parse() {
            req.headers_mut().insert(hyper::header::HOST, hv);
        }
    }

    let client = Client::builder(TokioExecutor::new()).build_http();

    match client.request(req).await {
        Ok(response) => response.into_response(),
        Err(e) => {
            tracing::error!("Proxy error: {e}");
            (StatusCode::BAD_GATEWAY, "Frontend server not available").into_response()
        }
    }
}
