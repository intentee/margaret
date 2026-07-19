use url::Url;

#[must_use]
pub fn unreachable_issuer_url() -> Url {
    Url::parse("https://127.0.0.1:1/.well-known/jwks.json")
        .expect("the unreachable issuer url parses")
}
