use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_server::svid_server_bundle::SvidServerBundle;

#[test]
fn provides_a_server_config() {
    let bundle = SvidServerBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    })
    .expect("the svid side is configured");

    assert!(bundle.server_config().alpn_protocols.is_empty());
}
