use serde_json::from_str;

use margaret_jwks_key_gen::persisted_jwks_secret::PersistedJwksSecret;

#[test]
fn persisted_jwks_secret_rejects_a_malformed_document() {
    assert!(from_str::<PersistedJwksSecret>(r#"{"current":{}}"#).is_err());
}
