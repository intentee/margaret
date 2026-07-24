use margaret_spiffe_svid_server::SvidServer;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn provides_a_server_config() {
    install_crypto_provider();

    let svid_server = SvidServer::new(
        "example.org".to_string(),
        "unix:///nonexistent.sock".to_string(),
    );

    assert!(svid_server.server_config().alpn_protocols.is_empty());
}
