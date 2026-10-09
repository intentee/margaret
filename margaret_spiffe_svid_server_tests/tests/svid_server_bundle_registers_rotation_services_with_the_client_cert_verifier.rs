use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_server::svid_server_bundle::SvidServerBundle;

#[tokio::test]
async fn registers_rotation_services_with_the_client_cert_verifier() {
    let bundle = SvidServerBundle::new(SvidServiceBundleParams {
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
