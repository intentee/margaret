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
async fn standard_verifier_resolves_a_rotated_key_set() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let claims = consumer_claims_expiring_at(FAR_FUTURE_EXPIRY);
    let signed_with_current = rotated.current.signing.sign(&claims).await?;
    let signed_with_previous = rotated.previous.signing.sign(&claims).await?;
    let published = PublicJwks::from(rotated);
    let published_kids: Vec<String> = published.keys.iter().map(|key| key.kid.clone()).collect();
    let document = serde_json::to_string(&published)?;

    let verifier = StandardVerifier::from_document(&document)?;

    assert_eq!(
        verifier.load_every_key()?,
        published_kids,
        "a consumer ingests the whole document, so every published key must load"
    );

    let from_current: ConsumerClaims =
        verifier.decode(&signed_with_current, &consumer_expected_claims())?;
    let from_previous: ConsumerClaims =
        verifier.decode(&signed_with_previous, &consumer_expected_claims())?;

    assert_eq!(from_current, claims);
    assert_eq!(from_previous, claims);

    Ok(())
}
