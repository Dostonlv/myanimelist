use aide::openapi::SecurityScheme;

pub const SECURITY_SCHEME_KEY: &str = "bearerAuth";

pub fn security_scheme() -> SecurityScheme {
    SecurityScheme::Http {
        scheme: "bearer".into(),
        bearer_format: Some("JWT".into()),
        description: None,
        extensions: Default::default(),
    }
}
