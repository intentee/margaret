use std::sync::Arc;

use rustls::RootCertStore;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_client::svid_server_cert_verifier_service::SvidServerCertVerifierService;

#[tokio::test]
async fn errors_when_root_store_is_empty() {
    let root_cert_store_holder = RootCertStoreHolder::default();
    root_cert_store_holder.set(Some(RootCertStore::empty()));

    let service = SvidServerCertVerifierService {
        crypto_provider: svid_crypto_provider(),
        root_cert_store_holder,
        spiffe_trust_domain: "example.org".to_string(),
        svid_server_cert_verifier_facade: Arc::new(SvidServerCertVerifierFacade::default()),
    };

    let result = Box::new(service).run(CancellationToken::new()).await;

    assert!(result.is_err());
}
