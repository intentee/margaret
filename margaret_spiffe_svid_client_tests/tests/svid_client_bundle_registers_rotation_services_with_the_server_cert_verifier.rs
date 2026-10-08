use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_client::svid_client_bundle::SvidClientBundle;

#[tokio::test]
async fn registers_rotation_services_with_the_server_cert_verifier() {
    let bundle = SvidClientBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    })
    .expect("the svid side is configured");

    let services = bundle
        .services()
        .await
        .expect("the bundle provides its services");

    assert_eq!(services.len(), 4);
}
