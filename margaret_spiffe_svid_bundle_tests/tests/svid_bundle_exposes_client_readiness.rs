use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

#[tokio::test]
async fn exposes_a_client_readiness_that_waits_for_the_svid() {
    let bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    })
    .expect("the svid side is configured");

    let mut readiness = bundle.client_readiness();
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    assert_eq!(
        readiness.wait_until_ready(&cancellation_token).await,
        SyncHolderPresence::Cancelled
    );
}
