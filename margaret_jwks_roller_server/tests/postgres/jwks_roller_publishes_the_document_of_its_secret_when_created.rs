use std::sync::Arc;

use bytes::Bytes;
use http::Method;

use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn jwks_roller_publishes_the_document_of_its_secret_when_created() {
    let started = started_with_signing_keys().await;
    let roller = JwksRoller::create(
        Arc::clone(&started.database),
        Arc::new(FixtureRsaSigningKeys::default()),
    )
    .await
    .expect("the roller starts");

    assert!(matches!(
        roller
            .public_jwks_handler()
            .handle(&FixtureRequest::new(Method::GET, "/.well-known/jwks.json").into_request())
            .await,
        Ok(ResponseContinuation::Done(response)) if response.status() == 200
    ));
    assert_eq!(
        roller.jwks_document_holder().get(),
        Bytes::from(
            serde_json::to_vec(roller.jwks_secret_holder().get().public_jwks())
                .expect("the public key set serializes")
        )
    );
}
