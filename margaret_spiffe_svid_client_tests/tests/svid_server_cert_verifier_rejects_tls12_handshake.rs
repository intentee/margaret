use std::sync::Arc;

use rustls::ClientConfig;
use rustls::ClientConnection;
use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::ServerName;
use rustls::version::TLS12;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::ca_der::CA_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_key_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;
use margaret_spiffe_svid_tests::pump_tls_handshake::pump_tls_handshake;

#[test]
fn rejects_tls12_when_used_as_client_server_verifier() {
    let mut root_store = RootCertStore::empty();
    root_store
        .add(CertificateDer::from(CA_DER.to_vec()))
        .unwrap();

    let svid_server_verifier =
        SvidServerCertVerifier::new(root_store, "example.org", svid_crypto_provider()).unwrap();

    let server_config = ServerConfig::builder_with_provider(svid_crypto_provider())
        .with_protocol_versions(&[&TLS12])
        .expect("the provider supports TLS 1.2")
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(
                LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec(),
            )],
            PrivateKeyDer::try_from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER.to_vec()).unwrap(),
        )
        .unwrap();

    let client_config = ClientConfig::builder_with_provider(svid_crypto_provider())
        .with_protocol_versions(&[&TLS12])
        .expect("the provider supports TLS 1.2")
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
