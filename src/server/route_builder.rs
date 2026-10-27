use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use hyper::upgrade::OnUpgrade;
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use crate::config::AppConfig;

type ProxyClient = Client<HttpConnector, Body>;

pub fn register_routes(proxy_url: Option<&str>, config: &AppConfig) -> Router {
    // tower-governor's `per_second` sets the token *replenish interval* to that many
    // seconds (so `per_second(2)` = one token every 2s = 0.5 req/s). Configure the
    // interval in milliseconds instead so `rate_limit_per_second` means what it says.
    let replenish_ms = (1000 / config.rate_limit_per_second.max(1)).max(1);
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(replenish_ms)
        .burst_size(config.rate_limit_burst)
        .finish()
        .expect("invalid rate limit config");

    let api_routes = crate::api::routes().layer(GovernorLayer::new(governor_conf));
    let router = Router::new().nest("/api", api_routes);

    match proxy_url {
        Some(url) => {
            let url: Arc<str> = Arc::from(url);
            // Build the client once and reuse its connection pool across every proxied
            // request instead of allocating a fresh pool per request.
            let client: ProxyClient = Client::builder(TokioExecutor::new()).build_http();
            router.fallback(move |req| {
                let url = Arc::clone(&url);
                let client = client.clone();
                async move { proxy_to_frontend(url, client, req).await }
            })
        }
        None => router,
    }
}

const DEV_HMR: [&str; 2] = ["/_next/hmr", "/_next/webpack-hmr"];
const DEV_ENDPOINTS: [&str; 9] = [
    "/__nextjs_attach-nodejs-inspector",
    "/__nextjs_devtools_config",
    "/__nextjs_disable_dev_indicator",
    "/__nextjs_launch-editor",
    "/__nextjs_original-stack-frame",
    "/__nextjs_original-stack-frames",
    "/__nextjs_restart_dev",
    "/__nextjs_server_status",
    "/__nextjs_source-map",
];

// The frontend only serves pages, so refuse anything else before it reaches Next.js:
// server action calls (`Next-Action`, the React2Shell probe shape), protocol upgrades
// other than the dev-server HMR socket, and methods other than GET/HEAD (except the
// dev overlay's own endpoints). The API under /api is routed separately and unaffected.
fn frontend_refusal(req: &Request) -> Option<Response> {
    let path = req.uri().path();
    let headers = req.headers();
    if headers.contains_key("next-action") {
        return Some(StatusCode::NOT_FOUND.into_response());
    }
    if headers.contains_key(hyper::header::UPGRADE) && !DEV_HMR.contains(&path) {
        return Some(StatusCode::NOT_FOUND.into_response());
    }
    match *req.method() {
        hyper::Method::GET | hyper::Method::HEAD => None,
        _ if DEV_ENDPOINTS.contains(&path) => None,
        _ => Some(
            (
                StatusCode::METHOD_NOT_ALLOWED,
                [(hyper::header::ALLOW, "GET, HEAD")],
            )
                .into_response(),
        ),
    }
}

async fn proxy_to_frontend(proxy_url: Arc<str>, client: ProxyClient, mut req: Request) -> Response {
    if let Some(refused) = frontend_refusal(&req) {
        return refused;
    }
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

    // Capture the client-side upgrade handle before forwarding so we can splice the two
    // connections together if the frontend answers with a protocol upgrade (e.g.
    // WebSockets, including the Next.js dev-server HMR socket).
    let client_upgrade = req.extensions_mut().remove::<OnUpgrade>();

    match client.request(req).await {
        Ok(mut response) => {
            if response.status() == StatusCode::SWITCHING_PROTOCOLS
                && let Some(client_upgrade) = client_upgrade
            {
                let backend_upgrade = hyper::upgrade::on(&mut response);
                tokio::spawn(async move {
                    match (client_upgrade.await, backend_upgrade.await) {
                        (Ok(client_io), Ok(backend_io)) => {
                            let mut client_io = TokioIo::new(client_io);
                            let mut backend_io = TokioIo::new(backend_io);
                            let _ = tokio::io::copy_bidirectional(&mut client_io, &mut backend_io)
                                .await;
                        }
                        (Err(e), _) | (_, Err(e)) => tracing::error!("Upgrade error: {e}"),
                    }
                });
            }
            response.into_response()
        }
        Err(e) => {
            tracing::error!("Proxy error: {e}");
            (StatusCode::BAD_GATEWAY, "Frontend server not available").into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(method: &str, path: &str, headers: &[(&str, &str)]) -> Request {
        let mut builder = Request::builder().method(method).uri(path);
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        builder.body(Body::empty()).expect("request")
    }

    fn status(r: Request) -> Option<StatusCode> {
        frontend_refusal(&r).map(|res| res.status())
    }

    #[test]
    fn the_frontend_proxy_only_forwards_page_traffic() {
        assert_eq!(status(req("GET", "/", &[])), None);
        assert_eq!(status(req("HEAD", "/wasm", &[("rsc", "1")])), None);
        for hmr in DEV_HMR {
            assert_eq!(status(req("GET", hmr, &[("upgrade", "websocket")])), None);
        }
        assert_eq!(
            status(req("POST", "/__nextjs_original-stack-frames", &[])),
            None
        );
        for probe in [
            req("POST", "/", &[("next-action", "x")]),
            req(
                "GET",
                "/",
                &[("next-action", "71039463937fee66205500a659a835c340716e9a")],
            ),
            req("GET", "/", &[("upgrade", "websocket")]),
            req("GET", "/x", &[("upgrade", "h2c")]),
        ] {
            assert_eq!(status(probe), Some(StatusCode::NOT_FOUND));
        }
        for sneaky in [
            "/__nextjs_x/../",
            "/__nextjs_source-map/../../",
            "/__nextjs_source%2Dmap",
            "/__nextjs_",
        ] {
            assert_eq!(
                status(req("POST", sneaky, &[])),
                Some(StatusCode::METHOD_NOT_ALLOWED)
            );
        }
        for method in ["POST", "PUT", "DELETE", "PATCH"] {
            assert_eq!(
                status(req(method, "/", &[])),
                Some(StatusCode::METHOD_NOT_ALLOWED)
            );
        }
    }
}
