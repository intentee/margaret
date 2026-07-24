use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn jwks_client_verifier_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_jwks_client".to_string(),
        "public_jwks_verifier".to_string(),
        "PublicJwksVerifier".to_string(),
    ])
}
