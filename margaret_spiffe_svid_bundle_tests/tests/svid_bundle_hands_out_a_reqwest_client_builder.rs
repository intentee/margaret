use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;

#[test]
fn hands_out_a_client_builder_with_the_svid_tls_so_callers_can_add_their_policy() {
    let bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    })
    .expect("the svid side is configured");

    assert!(
        bundle
            .reqwest_client_builder()
            .https_only(true)
            .build()
            .is_ok()
    );
}
