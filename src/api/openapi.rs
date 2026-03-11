use axum::{Router, response::IntoResponse, response::Json, routing::get};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::hello::handler,
        super::greet::handler,
        super::search::handler,
        super::create::handler,
        super::env::handler,
    ),
    components(schemas(
        super::ApiResponse,
        super::search::Params,
        super::create::Payload,
    )),
    info(
        title = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        description = env!("CARGO_PKG_DESCRIPTION"),
    )
)]
pub struct ApiDoc;

pub fn routes() -> Router {
    let router = Router::new().route("/openapi.json", get(openapi_json));

    if std::env::var("SWAGGER_UI").is_ok_and(|v| v == "false" || v == "0") {
        router
    } else {
        use utoipa_swagger_ui::SwaggerUi;
        tracing::info!("Swagger UI enabled at /api/swagger-ui");
        router.merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()))
    }
}

async fn openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}
