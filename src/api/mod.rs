pub mod anime;
pub mod extractor;
pub mod openapi;

use std::sync::Arc;

use aide::{
    axum::{IntoApiResponse, routing::get},
    openapi::OpenApi,
    swagger::Swagger,
};
use axum::{Extension, Json, Router, response::IntoResponse};
use tower_http::cors;

use crate::State;

pub async fn create_app(state: State) -> anyhow::Result<Router> {
    aide::generate::extract_schemas(true);

    let mut api = OpenApi::default();

    let app = aide::axum::ApiRouter::new()
        .nest("/animes", anime::router(state.clone()))
        .route("/api.json", get(serve_docs))
        .route(
            "/swagger",
            get(Swagger::new("/api.json")
                .with_title("myAnimelist")
                .axum_handler()),
        )
        .finish_api_with(&mut api, |op| {
            op.security_scheme(openapi::SECURITY_SCHEME_KEY, openapi::security_scheme())
        })
        .layer(Extension(Arc::new(api)))
        .layer(
            cors::CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_headers(cors::Any),
        )
        .with_state(state);
    Ok(app)
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
