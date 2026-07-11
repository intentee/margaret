use margaret_spiffe_svid_manager::extract_server_credentials::extract_server_credentials;
use margaret_spiffe_svid_manager_tests::build_workload_svid::build_workload_svid;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use rustls::SignatureScheme;

#[test]
fn produces_valid_certified_key() {
    install_crypto_provider();

    let credentials = extract_server_credentials(&build_workload_svid()).unwrap();

    assert_eq!(credentials.cert_chain.len(), 2);
    assert!(
        credentials
            .certified_key
            .key
            .choose_scheme(&[SignatureScheme::ECDSA_NISTP256_SHA256])
            .is_some()
    );
}
