use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use serde::Serialize;
use serde::Serializer;

struct UnserializableClaims;

impl Serialize for UnserializableClaims {
    fn serialize<TSerializer: Serializer>(
        &self,
        _serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        Err(<TSerializer::Error as serde::ser::Error>::custom(
            "claims cannot be serialized",
        ))
    }
}

#[tokio::test]
async fn sign_errors_on_unserializable_claims() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "encoding-kid".to_string(),
    })?;

    let result = keypair.signing.sign(&UnserializableClaims).await;

    assert!(matches!(result, Err(JwksKeyError::ClaimsJson { .. })));

    Ok(())
}
