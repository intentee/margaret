use std::sync::Arc;

use rustls::ClientConnection;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;

use margaret_spiffe_svid_manager_tests::build_mtls_client_config::build_mtls_client_config;
use margaret_spiffe_svid_manager_tests::build_mtls_server_config::build_mtls_server_config;
use margaret_spiffe_svid_manager_tests::handshake_error::HandshakeError;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_client_key_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_other_org_server_der::LEAF_SPIFFE_OTHER_ORG_SERVER_DER;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_other_org_server_key_der::LEAF_SPIFFE_OTHER_ORG_SERVER_KEY_DER;
use margaret_spiffe_svid_manager_tests::pump_tls_handshake::pump_tls_handshake;

#[test]
fn mtls_handshake_fails_with_wrong_trust_domain() {
    install_crypto_provider();

    let server_config = build_mtls_server_config(
        LEAF_SPIFFE_OTHER_ORG_SERVER_DER,
        LEAF_SPIFFE_OTHER_ORG_SERVER_KEY_DER,
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

    assert!(
        matches!(
            result,
            Err(HandshakeError::Tls(rustls::Error::InvalidCertificate(_))),
        ),
        "expected TLS certificate error, got: {result:?}",
    );
}
