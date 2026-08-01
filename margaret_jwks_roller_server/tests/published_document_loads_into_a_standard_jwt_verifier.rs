use std::sync::Arc;

use anyhow::Result;

use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwks_keygen_tests::consumer_claims::ConsumerClaims;
use margaret_jwks_keygen_tests::consumer_claims_expiring_at::consumer_claims_expiring_at;
use margaret_jwks_keygen_tests::consumer_expected_claims::consumer_expected_claims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::standard_verifier::StandardVerifier;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;

#[tokio::test]
async fn published_document_loads_into_a_standard_jwt_verifier() -> Result<()> {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(MemoryJwksSecretStorage),
    });

    bundle.roll_and_publish()?;

    let secret = bundle
        .jwks_secret_holder()
        .get()
        .expect("the first roll seeds a secret");
    let claims = consumer_claims_expiring_at(FAR_FUTURE_EXPIRY);
    let token = secret.current.signing.sign(&claims).await?;
    let document = bundle
        .jwks_document_holder()
        .get()
        .expect("the first roll publishes the document");

    let verifier = StandardVerifier::from_document(std::str::from_utf8(&document)?)?;

    assert!(!verifier.load_every_key()?.is_empty());

    let accepted: ConsumerClaims = verifier.decode(&token, &consumer_expected_claims())?;

    assert_eq!(accepted, claims);

    Ok(())
}
