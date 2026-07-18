use margaret_spiffe_svid::SvidServiceBundleParams;
use margaret_spiffe_svid_client::SvidClientBundle;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn builds_a_reqwest_client_from_the_client_config() {
    install_crypto_provider();

    let bundle = SvidClientBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    });

    assert!(bundle.reqwest_client().is_ok());
}
