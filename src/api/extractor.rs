use aide::{
    OperationInput,
    openapi::{Response, SecurityRequirement},
};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};

use crate::api::openapi;

pub struct Authorized(pub String);

impl<S> FromRequestParts<S> for Authorized
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(authorization) = parts.headers.get(AUTHORIZATION) else {
            tracing::info!("Not authorized");
            return Err((StatusCode::UNAUTHORIZED, "Not authorized"));
        };

        tracing::info!("Authorized {authorization:?}");

        Ok(Authorized(
            authorization.to_str().unwrap_or_default().to_string(),
        ))
    }
}

// We can do better:
// https://github.com/tamasfe/aide/blob/7bce6fe35c37a5a9aa8952fb94a137d7985f15bb/crates/aide/src/axum/inputs.rs#L72
impl OperationInput for Authorized {
    fn operation_input(
        _ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) {
        operation
            .security
            .append(&mut vec![SecurityRequirement::from([(
                openapi::SECURITY_SCHEME_KEY.into(),
                vec![],
            )])]);

        // let responses = operation.responses.get_or_insert_with(Default::default);
        // responses
        //     .responses
        //     .entry(aide::openapi::StatusCode::Code(401))
        //     .or_insert_with(|| {
        //         ReferenceOr::Item(Response {
        //             description: "Unauthorized".into(),
        //             ..Default::default()
        //         })
        //     });
    }

    fn inferred_early_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        vec![(
            Some(401),
            Response {
                description: "Unauthorized".into(),
                ..Default::default()
            },
        )]
    }
}
