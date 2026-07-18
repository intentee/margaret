use margaret_spiffe_svid::SvidServiceBundleParams;
use margaret_spiffe_svid_server::SvidServerBundle;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn provides_a_server_config() {
    install_crypto_provider();

    let bundle = SvidServerBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    });

    assert!(bundle.server_config().alpn_protocols.is_empty());
}
