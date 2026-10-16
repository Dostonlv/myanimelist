use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformOperation,
};
use axum::Json;

use crate::{State, api::extractor::Authorized, entities::animes::CreateAnime};

pub fn router(state: State) -> ApiRouter<State> {
    ApiRouter::new()
        .route("/", post_with(create_anime, create_anime_docs))
        .with_state(state)
}

#[tracing::instrument]
pub async fn create_anime(
    Authorized(_user): Authorized,
    Json(_request): Json<CreateAnime>,
) -> String {
    "ok".to_string()
}

pub fn create_anime_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}
