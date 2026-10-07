use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use serde_json::Value;
use serde_json::json;

use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::token_admission::TokenAdmission;
use margaret_issuer_directory_tests::counted_handler::CountedHandler;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_discovered_issuer::LOCALHOST_DISCOVERED_ISSUER;
use margaret_issuer_directory_tests::localhost_discovery::localhost_discovery;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_directory_tests::polled_fixture::PolledFixture;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

async fn admits_a_token_addressed_to(key: &FixtureRsaKey, trusted_issuer: &TrustedIssuer) -> bool {
    let token = key.token(
        &key.header(),
        &json!({
            "aud": trusted_issuer.trust.audience,
            "exp": 9_999_999_999_i64,
            "iat": 1_700_000_000,
            "iss": trusted_issuer.trust.issuer,
        }),
    );
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) = route_bearer_token(&authorization, &[trusted_issuer])
        .expect("the system clock reads as a numeric date")
    else {
        return false;
    };

    matches!(
        routed.admit::<Value, IdTokenProfile>(trusted_issuer).await,
        TokenAdmission::Admitted(_)
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn polls_an_issuer_trusted_by_two_declarations_once() {
    let key = FixtureRsaKey::load("rsa-kid");
    let discoveries = Arc::new(AtomicUsize::new(0));
    let key_set_fetches = Arc::new(AtomicUsize::new(0));
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: Arc::new(CountedHandler {
            handled: Arc::clone(&discoveries),
            inner: json_handler(200, &localhost_discovery()),
        }),
        key_set: Arc::new(CountedHandler {
            handled: Arc::clone(&key_set_fetches),
            inner: json_handler(200, &json!({ "keys": [key.jwk()] })),
        }),
    })
    .await;
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let polled = PolledFixture::discovered(LOCALHOST_DISCOVERED_ISSUER, Arc::clone(&metadata));
    let trusted_issuers = [
        localhost_trust(),
        TokenTrust {
            audience: "elsewhere",
            ..localhost_trust()
        },
    ]
    .map(|trust| polled.trusted(trust));
    let snapshot = polled.key_set.snapshot();
    let directory =
        PolledDirectory::start(vec![Arc::clone(&polled.polled)], issuer.request_client());

    polled.key_set.refreshed_since(&snapshot).await;
    directory.stop().await;
    issuer.stop().await;

    for trusted_issuer in &trusted_issuers {
        assert!(admits_a_token_addressed_to(&key, trusted_issuer).await);
    }
    assert!(matches!(metadata.holding(), MetadataHolding::Held(_)));
    assert_eq!(discoveries.load(Ordering::SeqCst), 1);
    assert_eq!(key_set_fetches.load(Ordering::SeqCst), 1);
}
