use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;

#[tokio::test]
async fn registers_rotation_services_once_with_a_verifier_per_direction() {
    let bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    })
    .expect("the svid side is configured");

    let services = bundle
        .services()
        .await
        .expect("the bundle provides its services");

    assert_eq!(services.len(), 5);
}
