use std::sync::Arc;

use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::pump_tls_handshake::pump_tls_handshake;
use margaret_spiffe_svid_manager_tests::test_fixtures::CA_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;
use rustls::ClientConfig;
use rustls::ClientConnection;
use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::ServerName;

#[test]
fn returns_error_when_server_receives_garbage() {
    install_crypto_provider();

    let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(
                LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec(),
            )],
            PrivateKeyDer::try_from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER.to_vec()).unwrap(),
        )
        .unwrap();
    let mut server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();

    let garbage: &[u8] = &[0xff; 16];
    server_connection.read_tls(&mut &garbage[..]).unwrap();

    let mut root_store = RootCertStore::empty();
    root_store
        .add(CertificateDer::from(CA_DER.to_vec()))
        .unwrap();
    let client_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let mut client_connection = ClientConnection::new(
        Arc::new(client_config),
        ServerName::try_from("ignored.example.org").unwrap(),
    )
    .unwrap();

    let result = pump_tls_handshake(&mut server_connection, &mut client_connection);

    assert!(result.is_err());
}
