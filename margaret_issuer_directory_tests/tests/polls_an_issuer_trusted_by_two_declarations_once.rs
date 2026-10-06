use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use serde_json::json;

use margaret_issuer_directory_tests::counted_handler::CountedHandler;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_discovery::localhost_discovery;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(flavor = "multi_thread")]
async fn polls_an_issuer_trusted_by_two_declarations_once() {
    let discoveries = Arc::new(AtomicUsize::new(0));
    let key_set_fetches = Arc::new(AtomicUsize::new(0));
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: Arc::new(CountedHandler {
            handled: Arc::clone(&discoveries),
            inner: json_handler(200, &localhost_discovery()),
        }),
        key_set: Arc::new(CountedHandler {
            handled: Arc::clone(&key_set_fetches),
            inner: json_handler(
                200,
                &json!({ "keys": [FixtureRsaKey::load("rsa-kid").jwk()] }),
            ),
        }),
    })
    .await;
    let metadata = [
        Arc::new(IssuerMetadata::awaiting()),
        Arc::new(IssuerMetadata::awaiting()),
    ];
    let trusts = [
        localhost_trust(),
        TokenTrust {
            audience: "elsewhere".parse().expect("the audience is not empty"),
            ..localhost_trust()
        },
    ];
    let trusted_issuers = [0, 1].map(|member| {
        Arc::new(TrustedIssuer::for_oidc_issuer(
            Arc::clone(&metadata[member]),
            Arc::new(TokenTrustDeclaration {
                trust: trusts[member].clone(),
            }),
        ))
    });
    let snapshot = trusted_issuers[1].key_set.snapshot();
    let directory = PolledDirectory::start(trusted_issuers.to_vec(), issuer.request_client());

    assert!(matches!(
        trusted_issuers[1].key_set.refreshed_since(&snapshot).await,
        KeySetRefresh::Refreshed(KeySetHolding::Held(_))
    ));

    directory.stop().await;
    issuer.stop().await;

    assert!(matches!(
        trusted_issuers[0].key_set.snapshot().holding,
        KeySetHolding::Held(_)
    ));
    assert!(
        metadata
            .iter()
            .all(|metadata| matches!(metadata.holding(), MetadataHolding::Held(_)))
    );
    assert_eq!(discoveries.load(Ordering::SeqCst), 1);
    assert_eq!(key_set_fetches.load(Ordering::SeqCst), 1);
}
