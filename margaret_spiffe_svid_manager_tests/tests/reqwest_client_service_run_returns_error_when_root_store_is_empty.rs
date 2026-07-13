use std::sync::Arc;

use rustls::RootCertStore;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_spiffe_svid_manager::reqwest_client_holder::ReqwestClientHolder;
use margaret_spiffe_svid_manager::reqwest_client_service::ReqwestClientService;
use margaret_spiffe_svid_manager::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_manager::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid_manager_tests::build_workload_credentials::build_workload_credentials;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;

#[tokio::test]
async fn run_returns_error_when_root_store_is_empty() {
    install_crypto_provider();

    let root_cert_store_holder = RootCertStoreHolder::default();
    let svid_certified_key_holder = SvidCertifiedKeyHolder::default();

    root_cert_store_holder.set(Some(RootCertStore::empty()));
    svid_certified_key_holder.set(Some(Arc::new(build_workload_credentials())));

    let service = ReqwestClientService {
        reqwest_client_holder: ReqwestClientHolder::default(),
        root_cert_store_holder,
        spiffe_trust_domain: "example.org".to_string(),
        svid_certified_key_holder,
    };

    let result = Box::new(service).run(CancellationToken::new()).await;

    assert!(result.is_err());
}
