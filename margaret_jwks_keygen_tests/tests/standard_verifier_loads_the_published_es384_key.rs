use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwks_keygen_tests::consumer_claims::ConsumerClaims;
use margaret_jwks_keygen_tests::consumer_claims_expiring_at::consumer_claims_expiring_at;
use margaret_jwks_keygen_tests::consumer_expected_claims::consumer_expected_claims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::standard_verifier::StandardVerifier;

#[tokio::test]
async fn standard_verifier_loads_the_published_es384_key() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P384)?;
    let claims = consumer_claims_expiring_at(FAR_FUTURE_EXPIRY);
    let token = secret.current.signing.sign(&claims).await?;
    let document = serde_json::to_string(&PublicJwks::from(secret))?;

    let verified: ConsumerClaims = StandardVerifier::from_document(&document)?
        .decode(&token, &consumer_expected_claims())?;

    assert_eq!(verified, claims);

    Ok(())
}
