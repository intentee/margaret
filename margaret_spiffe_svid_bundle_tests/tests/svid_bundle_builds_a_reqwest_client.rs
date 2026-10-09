use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;

#[test]
fn builds_a_reqwest_client() {
    let bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    })
    .expect("the svid side is configured");

    assert!(bundle.reqwest_client().is_ok());
}
