use std::sync::Arc;

use bytes::Bytes;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn jwks_roller_publishes_the_document_of_its_secret_when_created() {
    let roller = JwksRoller::create(
        Arc::new(FixtureSigningKeys::empty()),
        Arc::new(FixtureRsaSigningKeys::default()),
    )
    .await
    .expect("the roller starts");

    assert_eq!(roller.public_jwks_handler().respond().status(), 200);
    assert_eq!(
        roller.jwks_document_holder().get(),
        Bytes::from(
            serde_json::to_vec(roller.jwks_secret_holder().get().public_jwks())
                .expect("the public key set serializes")
        )
    );
}
