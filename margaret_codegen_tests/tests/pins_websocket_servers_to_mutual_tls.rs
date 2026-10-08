use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

#[test]
fn serves_a_websocket_only_server_over_spiffe_mutual_tls_when_its_session_reads_the_peer() {
    let generated = generate_fixture("websocket_peer_identity").expect("the fixture generates");
    let serve: String = generated_module_source(&generated, "serve")
        .expect("the serve module is generated")
        .split_whitespace()
        .collect();

    assert!(serve.contains(
        "margaret::framework::spiffe_svid_server::svid_server_bundle::SvidServerBundle::new("
    ));
    assert!(serve.contains(
        "address_argument:\"mesh-addr\",routes:super::http::server_mesh::server_mesh(container,routes),transport:matchmatches.get_one::<margaret::framework::service::transport_choice::TransportChoice"
    ));
    assert!(
        serve.contains(r#"("mesh-transport"){Some(value)=>value.config(spiffe_server_config),"#)
    );
}
