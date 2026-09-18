use url::Url;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_client::svid_client_bundle::SvidClientBundle;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_websocket_client::web_socket_client_error::WebSocketClientError;

#[tokio::test]
async fn builds_a_web_socket_client_that_refuses_plaintext_urls() {
    install_crypto_provider();

    let bundle = SvidClientBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: "example.org".to_string(),
        spire_agent_addr: "unix:///nonexistent.sock".to_string(),
    });
    let url = Url::parse("ws://localhost:9/ws").expect("the fixture url parses");

    assert!(matches!(
        match bundle.web_socket_client().connect(&url).await {
            Ok(_) => panic!("a plaintext url is refused"),
            Err(error) => error,
        },
        WebSocketClientError::UnusableUrl { .. }
    ));
}
