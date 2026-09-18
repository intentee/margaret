use margaret_codegen::generated_code::GeneratedCode;
use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

fn generated() -> GeneratedCode {
    generate_fixture("spiffe_clients").expect("the fixture generates")
}

fn module(generated: &GeneratedCode, name: &str) -> String {
    generated_module_source(generated, name)
        .expect("the module is generated")
        .split_whitespace()
        .collect()
}

#[test]
fn binds_the_spiffe_websocket_client_from_the_bundle() {
    assert!(
        module(&generated(), "serve")
            .contains("letspiffe_websocket_client=spiffe_bundle.web_socket_client();")
    );
}

#[test]
fn binds_the_spiffe_http_client_alongside_the_websocket_client() {
    let source = module(&generated(), "serve");

    assert!(source.contains("letspiffe_http_client=matchspiffe_bundle.reqwest_client()"));
    assert!(
        source.contains(
            "margaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle"
        )
    );
}

#[test]
fn weaves_the_websocket_client_into_the_container_bootstrap() {
    assert!(
        module(&generated(), "container/build/serve_arguments")
            .contains("margaret::framework::websocket_client::web_socket_client::WebSocketClient")
    );
}
