use std::sync::Arc;

use margaret_spiffe_svid_manager::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_manager::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_manager::svid_client_cert_verifier_service::SvidClientCertVerifierService;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use rustls::RootCertStore;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

#[tokio::test]
async fn errors_when_root_store_is_empty() {
    install_crypto_provider();

    let root_cert_store_holder = RootCertStoreHolder::default();
    root_cert_store_holder.set(Some(RootCertStore::empty()));

    let service = SvidClientCertVerifierService {
        root_cert_store_holder,
        svid_client_cert_verifier: Arc::new(SvidClientCertVerifier::default()),
    };

    let result = Box::new(service).run(CancellationToken::new()).await;

    assert!(result.is_err());
}
