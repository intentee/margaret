use margaret_codegen::generated_code::GeneratedCode;
use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

fn module(generated: &GeneratedCode, name: &str) -> String {
    generated_module_source(generated, name)
        .expect("the module is generated")
        .split_whitespace()
        .collect()
}

fn generated(fixture: &str) -> GeneratedCode {
    generate_fixture(fixture).expect("the fixture generates")
}

#[test]
fn pins_the_transport_when_a_session_reads_the_peer_spiffe_id() {
    assert!(
        module(&generated("websocket_peer_identity"), "serve").contains(
            "transport:margaret::framework::http::transport_config::TransportConfig::MutualTls"
        )
    );
}

#[test]
fn declares_the_spiffe_arguments_when_a_session_reads_the_peer_spiffe_id() {
    assert!(
        module(&generated("websocket_peer_identity"), "run").contains("\"spiffe-trust-domain\"")
    );
}

#[test]
fn pins_the_transport_when_a_session_middleware_reads_the_peer_spiffe_id() {
    assert!(
        module(&generated("websocket_peer_identity_middleware"), "serve").contains(
            "transport:margaret::framework::http::transport_config::TransportConfig::MutualTls"
        )
    );
}
