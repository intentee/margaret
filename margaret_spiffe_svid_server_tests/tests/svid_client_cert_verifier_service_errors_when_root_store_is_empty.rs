use std::sync::Arc;

use rustls::RootCertStore;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use margaret_spiffe_svid_server::svid_client_cert_verifier_service::SvidClientCertVerifierService;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[tokio::test]
async fn errors_when_root_store_is_empty() {
    install_crypto_provider();

    let root_cert_store_holder = RootCertStoreHolder::default();
    root_cert_store_holder.set(Some(RootCertStore::empty()));

    let service = SvidClientCertVerifierService {
        root_cert_store_holder,
        spiffe_trust_domain: "example.org".to_string(),
        svid_client_cert_verifier_facade: Arc::new(SvidClientCertVerifierFacade::default()),
    };

    let result = Box::new(service).run(CancellationToken::new()).await;

    assert!(result.is_err());
}
