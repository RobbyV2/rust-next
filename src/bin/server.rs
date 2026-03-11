use std::net::SocketAddr;

use axum::http::{HeaderName, Method, header};
use clap::Parser;
use rust_next::config::{AppConfig, AppMode, CliOverrides};
use rust_next::server::build_router;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::from_filename(".env.local");
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = CliOverrides::parse();
    let config = AppConfig::load(&cli)?;

    // DB integration point: initialize your database pool here
    // e.g. let pool = sqlx::PgPool::connect(config.database_url.as_deref().unwrap_or("...")).await?;
    // Then pass it as shared state: build_router(proxy_url.as_deref()).with_state(pool)

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::UPGRADE,
            header::CONNECTION,
            HeaderName::from_static("sec-websocket-key"),
            HeaderName::from_static("sec-websocket-version"),
            HeaderName::from_static("sec-websocket-protocol"),
        ])
        .allow_credentials(true);

    let proxy_url = config.proxy_url();
    let app = build_router(proxy_url.as_deref(), &config).layer(cors);

    let addr = config.addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    let mode_label = match config.app_mode {
        AppMode::Full => "full (proxy to frontend)",
        AppMode::ApiOnly => "api-only",
    };

    info!("Starting rust-next server [mode: {mode_label}]");
    info!("Listening on http://{addr}");

    if let Some(ref url) = proxy_url {
        info!("Proxying frontend requests to {url}");
    }

    info!(
        "Rate limiting: {} req/s, burst {}",
        config.rate_limit_per_second, config.rate_limit_burst
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C, shutting down..."),
        _ = terminate => info!("Received SIGTERM, shutting down..."),
    }
}
