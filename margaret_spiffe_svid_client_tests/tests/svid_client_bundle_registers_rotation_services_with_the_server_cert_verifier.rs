use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_client::svid_client_bundle::SvidClientBundle;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[tokio::test]
async fn registers_rotation_services_with_the_server_cert_verifier() {
    install_crypto_provider();

    let bundle = SvidClientBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    });

    let services = bundle
        .services()
        .await
        .expect("the bundle provides its services");

    assert_eq!(services.len(), 4);
}
