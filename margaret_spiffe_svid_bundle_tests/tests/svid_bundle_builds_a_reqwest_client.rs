use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn builds_a_reqwest_client() {
    install_crypto_provider();

    let bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    });

    assert!(bundle.reqwest_client().is_ok());
}
