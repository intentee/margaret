use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client::verification_key_set_holder::VerificationKeySetHolder;
use margaret_jwks_client_tests::failing_endpoint::FailingEndpoint;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::system_issuer_document_client::system_issuer_document_client;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[tokio::test(start_paused = true)]
async fn public_jwks_poll_service_retains_the_last_set_when_the_endpoint_is_unresolvable() {
    let KeySetParsing::Accepted(known_good) = VerificationKeySet::from_jwks(Vec::new()) else {
        panic!("an empty key set is accepted");
    };
    let known_good = Arc::new(known_good);
    let verification_key_set_holder = VerificationKeySetHolder::default();

    verification_key_set_holder.set(Some(known_good.clone()));

    let mut service = PublicJwksPollService {
        endpoint_provider: Arc::new(FailingEndpoint),
        issuer_document_client: system_issuer_document_client(),
        verification_key_set_holder: verification_key_set_holder.clone(),
    };

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed resolution never fails the service");

    let retained = verification_key_set_holder
        .get()
        .expect("the last known good document survives a failed resolution");

    assert!(Arc::ptr_eq(&retained, &known_good));
}
