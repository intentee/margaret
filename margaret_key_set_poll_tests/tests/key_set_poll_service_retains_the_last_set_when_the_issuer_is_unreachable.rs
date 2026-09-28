use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;
use url::Url;

use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::first_tick_context::first_tick_context;
use margaret_key_set_poll_tests::fixed_key_set_locator::FixedKeySetLocator;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test(start_paused = true)]
async fn key_set_poll_service_retains_the_last_set_when_the_issuer_is_unreachable() {
    let KeySetParsing::Accepted(known_good) = VerificationKeySet::from_jwks(Vec::new()) else {
        panic!("an empty key set is accepted");
    };
    let known_good = Arc::new(known_good);
    let verification_key_set_holder = VerificationKeySetHolder::default();

    verification_key_set_holder.set(Some(known_good.clone()));

    let mut service = KeySetPollService {
        issuer_document_client: system_issuer_document_client(),
        locator: FixedKeySetLocator {
            key_set_url: Url::parse("https://127.0.0.1:1").expect("the unreachable url parses"),
        },
        verification_key_set_holder: verification_key_set_holder.clone(),
    };

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    let retained = verification_key_set_holder
        .get()
        .expect("the last known good set survives a failed poll");

    assert!(Arc::ptr_eq(&retained, &known_good));
}
