use std::sync::Arc;

use rustls::ClientConnection;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;

use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_client_tests::build_client_config_with_svid_server_verifier::build_client_config_with_svid_server_verifier;
use margaret_spiffe_svid_tests::pump_tls_handshake::pump_tls_handshake;

#[test]
fn server_fails_when_holder_has_no_cert() {
    let holder = SvidCertifiedKeyHolder::default();

    let server_config = ServerConfig::builder_with_provider(svid_crypto_provider())
        .with_safe_default_protocol_versions()
        .expect("the provider supports the safe default protocol versions")
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(holder));
    let client_config = build_client_config_with_svid_server_verifier();

    let mut server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();
    let mut client_connection = ClientConnection::new(
        Arc::new(client_config),
        ServerName::try_from("ignored.example.org").unwrap(),
    )
    .unwrap();

    let result = pump_tls_handshake(&mut server_connection, &mut client_connection);

    assert!(result.is_err());
}
