use std::sync::Arc;

use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get}, openapi::OpenApi, swagger::Swagger,
};
use axum::{Extension, Json, response::IntoResponse};

use crate::State;

pub fn router(state: State) -> ApiRouter<State> {
    ApiRouter::new()
        .route(
            "/swagger",
            get(Swagger::new("/api.json")
                .with_title("Animes")
                .axum_handler()),
        )
        .route("/api.json", get(serve_docs))
        // .route("/", post_with(serve_docs))
        .with_state(state)
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}


