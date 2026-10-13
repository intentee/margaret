use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

#[test]
fn serves_a_websocket_only_server_over_spiffe_mutual_tls_when_its_session_reads_the_peer() {
    let generated = generate_fixture("websocket_peer_identity").expect("the fixture generates");
    let run: String = generated_module_source(&generated, "run")
        .expect("the run module is generated")
        .split_whitespace()
        .collect();
    let serve: String = generated_module_source(&generated, "serve")
        .expect("the serve module is generated")
        .split_whitespace()
        .collect();

    assert!(run.contains(
        "margaret::framework::spiffe_svid_server::svid_server_bundle::SvidServerBundle::new("
    ));
    assert!(run.contains(
        r#"margaret::framework::console::required_argument::required_argument::<margaret::framework::service::transport_choice::TransportChoice,>(matches,"mesh-transport").map(|server_mesh_transport|{letserver_mesh_listener=margaret::framework::service::server_listener::ServerListener{address:server_mesh_address,transport:server_mesh_transport.config(&spiffe_server_config),"#
    ));
    assert!(serve.contains(
        "margaret::framework::service::server_service::ServerService::new(server_mesh_listener,super::http::server_mesh::server_mesh(container,routes),)"
    ));
}
