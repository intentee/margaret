use anyhow::Result;
use serde::Serialize;
use serde::Serializer;
use serde::ser::Error;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;

struct UnserializableClaims;

impl Serialize for UnserializableClaims {
    fn serialize<TSerializer: Serializer>(
        &self,
        _serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        Err(<TSerializer::Error as Error>::custom(
            "claims cannot be serialized",
        ))
    }
}

#[tokio::test]
async fn sign_errors_on_unserializable_claims() -> Result<()> {
    let result = fixture_pair(Curve::P256, "encoding-kid")
        .sign(&UnserializableClaims)
        .await;

    assert!(matches!(
        result,
        Err(JwksKeyError::ClaimsSerialization { .. })
    ));

    Ok(())
}
