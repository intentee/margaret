use std::sync::Arc;

use rustls::ClientConfig;
use rustls::ClientConnection;
use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::ServerName;

use margaret_spiffe_svid_manager::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::pump_tls_handshake::pump_tls_handshake;
use margaret_spiffe_svid_manager_tests::test_fixtures::CA_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;

#[test]
fn rejects_tls12_when_used_as_client_server_verifier() {
    install_crypto_provider();

    let mut root_store = RootCertStore::empty();
    root_store
        .add(CertificateDer::from(CA_DER.to_vec()))
        .unwrap();

    let svid_server_verifier =
        SvidServerCertVerifier::new(root_store, "example.org".to_string()).unwrap();

    let server_config = ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(
                LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec(),
            )],
            PrivateKeyDer::try_from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER.to_vec()).unwrap(),
        )
        .unwrap();

    let client_config = ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(svid_server_verifier))
        .with_no_client_auth();

    let mut server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();
    let mut client_connection = ClientConnection::new(
        Arc::new(client_config),
        ServerName::try_from("ignored.example.org").unwrap(),
    )
    .unwrap();

    let result = pump_tls_handshake(&mut server_connection, &mut client_connection);

    assert!(result.is_err());
}
