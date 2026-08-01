use anyhow::Result;
use jsonwebtoken::errors::ErrorKind;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwks_keygen_tests::already_expired_expiry::ALREADY_EXPIRED_EXPIRY;
use margaret_jwks_keygen_tests::consumer_claims::ConsumerClaims;
use margaret_jwks_keygen_tests::consumer_claims_expiring_at::consumer_claims_expiring_at;
use margaret_jwks_keygen_tests::consumer_expected_claims::consumer_expected_claims;
use margaret_jwks_keygen_tests::standard_verifier::StandardVerifier;

#[tokio::test]
async fn standard_verifier_rejects_an_expired_consumer_token() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = secret
        .current
        .signing
        .sign(&consumer_claims_expiring_at(ALREADY_EXPIRED_EXPIRY))
        .await?;
    let document = serde_json::to_string(&PublicJwks::from(secret))?;

    let rejection = StandardVerifier::from_document(&document)?
        .decode::<ConsumerClaims>(&token, &consumer_expected_claims())
        .expect_err("an expired token is refused")
        .downcast::<jsonwebtoken::errors::Error>()
        .expect("the refusal comes from the standard verifier");

    assert!(
        matches!(rejection.kind(), ErrorKind::ExpiredSignature),
        "a standard verifier reads exp as a number, not as a string: {rejection:?}"
    );

    Ok(())
}
