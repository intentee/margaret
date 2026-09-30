use serde::Serialize;
use serde::Serializer;
use serde::ser::Error;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

struct Unserializable;

impl Serialize for Unserializable {
    fn serialize<Target: Serializer>(
        &self,
        _serializer: Target,
    ) -> Result<Target::Ok, Target::Error> {
        Err(Target::Error::custom("this value cannot be serialized"))
    }
}

#[test]
fn reports_client_assertion_claims_that_cannot_be_serialized() {
    assert!(matches!(
        rolled_store(fresh_p256_secret()).sign_client_assertion(&Unserializable),
        Err(JwksSecretStoreError::ClaimsSerialization { .. })
    ));
}
