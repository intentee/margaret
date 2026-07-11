use std::sync::Arc;

use margaret_spiffe_svid_manager_tests::build_mtls_client_config::build_mtls_client_config;
use margaret_spiffe_svid_manager_tests::build_mtls_server_config::build_mtls_server_config;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::pump_tls_handshake::pump_tls_handshake;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;
use rustls::ClientConnection;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;

#[test]
fn mtls_handshake_completes_with_matching_trust_domain() {
    install_crypto_provider();

    let server_config = build_mtls_server_config(
        LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER,
        LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER,
    );
    let client_config = build_mtls_client_config(
        LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER,
        LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER,
        "example.org",
    );

    let mut server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();
    let mut client_connection = ClientConnection::new(
        Arc::new(client_config),
        ServerName::try_from("ignored.example.org").unwrap(),
    )
    .unwrap();

    let result = pump_tls_handshake(&mut server_connection, &mut client_connection);

    assert!(result.is_ok());
    assert_eq!(
        client_connection.protocol_version(),
        Some(rustls::ProtocolVersion::TLSv1_3),
    );
}
