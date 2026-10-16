pub mod anime;

use std::sync::Arc;

use aide::openapi::OpenApi;
use axum::{Extension, Router};
use tower_http::cors;

use crate::State;

pub async fn create_app(state: State) -> anyhow::Result<Router> {
    aide::generate::extract_schemas(true);

    let mut api = OpenApi::default();

    let app = aide::axum::ApiRouter::new()
        .nest("/animes", anime::router(state.clone()))
        // .route(
        //     "/swagger",
        //     get(Swagger::new("/api.json")
        //         .with_title("myAnimelist")
        //         .axum_handler()),
        // )
        .finish_api_with(
            &mut api,
            |op| op, //  {
                     // op.security_scheme(
                     //     auth::openapi::SECURITY_SCHEME_KEY,
                     //     auth::openapi::security_scheme(),
                     // )
                     // }
        )
        .layer(Extension(Arc::new(api)))
        .layer(
            cors::CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_headers(cors::Any),
        )
        .with_state(state);
    Ok(app)
}
