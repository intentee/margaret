use std::sync::Arc;

use rustls::ClientConfig;
use rustls::ClientConnection;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::ServerName;

use margaret_spiffe_svid_manager::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_manager_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_manager_tests::build_webpki_client_verifier::build_webpki_client_verifier;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::pump_tls_handshake::pump_tls_handshake;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;

#[tokio::test]
async fn rejects_tls12_when_used_as_server_client_verifier() {
    install_crypto_provider();

    let svid_client_verifier = Arc::new(SvidClientCertVerifier::default());
    svid_client_verifier.update_internal_verifier(build_webpki_client_verifier());

    let server_config = ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
        .with_client_cert_verifier(svid_client_verifier)
        .with_single_cert(
            vec![CertificateDer::from(
                LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec(),
            )],
            PrivateKeyDer::try_from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER.to_vec()).unwrap(),
        )
        .unwrap();

    let client_config = ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
        .with_root_certificates(build_root_cert_store_with_ca())
        .with_client_auth_cert(
            vec![CertificateDer::from(
                LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec(),
            )],
            PrivateKeyDer::try_from(LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER.to_vec()).unwrap(),
        )
        .unwrap();

    let mut server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();
    let mut client_connection = ClientConnection::new(
        Arc::new(client_config),
        ServerName::try_from("ignored.example.org").unwrap(),
    )
    .unwrap();

    let result = pump_tls_handshake(&mut server_connection, &mut client_connection);

    assert!(result.is_err());
}
