use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformOperation,
};
use axum::{Json, http::StatusCode};

use crate::{
    State,
    api::AppResponse,
    entities::animes::{CreateAnime, Model},
};

pub fn router(state: State) -> ApiRouter<State> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_anime_genres, create_anime_genres_docs),
        )
        .with_state(state)
}

#[tracing::instrument]
pub async fn create_anime_genres(
    // Authorized(_user): Authorized,
    Json(_request): Json<CreateAnime>,
) -> AppResponse<Model> {
    AppResponse {
        status_code: StatusCode::OK.as_u16(),
        message: String::from("test"),
        data: None,
    }
}

pub fn create_anime_genres_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime Genres")
}
