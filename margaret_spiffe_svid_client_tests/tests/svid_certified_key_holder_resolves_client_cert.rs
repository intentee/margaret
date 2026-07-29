use std::sync::Arc;

use rustls::ClientConfig;
use rustls::ClientConnection;
use rustls::ServerConnection;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;

use margaret_spiffe_svid::build_svid_certified_key::build_svid_certified_key;
use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_mtls_server_config::build_mtls_server_config;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_key_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_key_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER;
use margaret_spiffe_svid_tests::pump_tls_handshake::pump_tls_handshake;

#[test]
fn client_presents_cert_resolved_through_holder() {
    install_crypto_provider();

    let credentials = build_svid_certified_key(
        vec![CertificateDer::from(
            LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec(),
        )],
        LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER,
    )
    .unwrap();

    let holder = SvidCertifiedKeyHolder::default();
    holder.set(Some(Arc::new(credentials)));

    let client_config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(
            SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org")
                .unwrap(),
        ))
        .with_client_cert_resolver(Arc::new(holder));
    let server_config = build_mtls_server_config(
        LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER,
        LEAF_SPIFFE_EXAMPLE_ORG_SERVER_KEY_DER,
    );

    let mut server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();
    let mut client_connection = ClientConnection::new(
        Arc::new(client_config),
        ServerName::try_from("ignored.example.org").unwrap(),
    )
    .unwrap();

    let result = pump_tls_handshake(&mut server_connection, &mut client_connection);

    assert!(result.is_ok(), "handshake failed: {result:?}");
}
