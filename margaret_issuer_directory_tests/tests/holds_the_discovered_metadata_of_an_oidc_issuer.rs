use std::sync::Arc;

use serde_json::json;

use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(flavor = "multi_thread")]
async fn holds_the_discovered_metadata_of_an_oidc_issuer() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(
            200,
            &json!({
                "issuer": "https://localhost",
                "jwks_uri": "https://localhost/jwks",
                "token_endpoint": "https://localhost/token",
            }),
        ),
        key_set: json_handler(200, &json!({ "keys": [] })),
    })
    .await;
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let trusted_issuer = Arc::new(TrustedIssuer::for_oidc_issuer(
        Arc::clone(&metadata),
        Arc::new(localhost_trust()),
    ));

    first_poll(&trusted_issuer, issuer.request_client()).await;
    issuer.stop().await;

    let MetadataHolding::Held(held) = metadata.holding() else {
        panic!("the discovered metadata is held");
    };

    assert_eq!(
        held.token_endpoint,
        AdvertisedEndpoint::Advertised(
            "https://localhost/token"
                .parse()
                .expect("the fixture endpoint is a url")
        )
    );
}
