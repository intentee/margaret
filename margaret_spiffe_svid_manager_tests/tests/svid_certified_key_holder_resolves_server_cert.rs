use std::sync::Arc;

use rustls::ClientConnection;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;
use spiffe::X509Svid;

use margaret_spiffe_svid_manager::extract_server_credentials::extract_server_credentials;
use margaret_spiffe_svid_manager::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid_manager_tests::build_client_config_with_svid_server_verifier::build_client_config_with_svid_server_verifier;
use margaret_spiffe_svid_manager_tests::ca_der::CA_DER;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_server_key_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;
use margaret_spiffe_svid_manager_tests::pump_tls_handshake::pump_tls_handshake;

#[tokio::test]
async fn server_serves_cert_resolved_through_holder() {
    install_crypto_provider();

    let cert_chain_der = [LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER, CA_DER].concat();
    let svid =
        X509Svid::parse_from_der(&cert_chain_der, LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER).unwrap();
    let credentials = extract_server_credentials(&svid).unwrap();

    let holder = SvidCertifiedKeyHolder::default();
    holder.set(Some(Arc::new(credentials)));

    let server_config = ServerConfig::builder()
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

    assert!(result.is_ok(), "handshake failed: {result:?}");
}
