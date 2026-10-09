use std::sync::Arc;

use rustls::ClientConnection;
use rustls::ProtocolVersion;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;

use margaret_spiffe_svid_client_tests::build_mtls_client_config::build_mtls_client_config;
use margaret_spiffe_svid_tests::build_mtls_server_config::build_mtls_server_config;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_key_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_key_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;
use margaret_spiffe_svid_tests::pump_tls_handshake::pump_tls_handshake;

#[test]
fn mtls_handshake_completes_with_matching_trust_domain() {
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
        Some(ProtocolVersion::TLSv1_3),
    );
}
