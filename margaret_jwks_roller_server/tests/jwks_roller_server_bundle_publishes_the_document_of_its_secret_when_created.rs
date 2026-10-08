use std::sync::Arc;

use bytes::Bytes;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn jwks_roller_server_bundle_publishes_the_document_of_its_secret_when_created() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(FixtureSigningKeys::empty()),
    })
    .await
    .expect("the first secret is rolled and published");

    assert_eq!(
        bundle.jwks_document_holder().get(),
        Bytes::from(
            serde_json::to_vec(bundle.jwks_secret_holder().get().public_jwks())
                .expect("the public key set serializes")
        )
    );
}
