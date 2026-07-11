use std::sync::Arc;

use margaret_spiffe_svid_manager::build_rustls_client_config::build_rustls_client_config;
use margaret_spiffe_svid_manager_tests::build_workload_credentials::build_workload_credentials;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use rustls::RootCertStore;

#[test]
fn fails_when_root_store_is_empty() {
    install_crypto_provider();

    let result = build_rustls_client_config(
        RootCertStore::empty(),
        Arc::new(build_workload_credentials()),
        "example.org".to_string(),
    );

    assert!(result.is_err());
}
