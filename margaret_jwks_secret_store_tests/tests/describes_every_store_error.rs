use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;

#[test]
fn describes_every_store_error() {
    assert!(
        JwksSecretStoreError::ClaimsSerialization {
            source: serde_json::from_str::<u8>("x").expect_err("not json"),
        }
        .to_string()
        .starts_with("the claims to sign could not be serialized to json: ")
    );
    assert_eq!(
        JwksSecretStoreError::SecretUnavailable.to_string(),
        "the signing secret is not available yet"
    );
}
