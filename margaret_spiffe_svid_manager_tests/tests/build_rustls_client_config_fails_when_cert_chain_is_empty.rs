use std::sync::Arc;

use margaret_spiffe_svid_manager::build_rustls_client_config::build_rustls_client_config;
use margaret_spiffe_svid_manager::svid_certified_key::SvidCertifiedKey;
use margaret_spiffe_svid_manager_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_manager_tests::build_workload_credentials::build_workload_credentials;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn fails_when_cert_chain_is_empty() {
    install_crypto_provider();

    let valid = build_workload_credentials();
    let svid_key_without_cert_chain = SvidCertifiedKey {
        cert_chain: vec![],
        certified_key: valid.certified_key.clone(),
        private_key_der: valid.private_key_der.clone_key(),
    };

    let result = build_rustls_client_config(
        build_root_cert_store_with_ca(),
        Arc::new(svid_key_without_cert_chain),
        "example.org".to_string(),
    );

    assert!(result.is_err());
}
