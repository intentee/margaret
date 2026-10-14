use spiffe::X509BundleSet;
use spiffe::X509Context;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid::svid_converter_service::SvidConverterService;
use margaret_spiffe_svid::svid_error::SvidError;

#[tokio::test]
async fn stops_on_a_context_it_cannot_convert_so_stale_credentials_are_never_served() {
    let (ca_bundle_tx, _ca_bundle_rx) = broadcast::channel(1);
    let (x509_context_tx, x509_context_rx) = broadcast::channel(1);
    let service = SvidConverterService {
        ca_bundle_tx,
        svid_certified_key_holder: SvidCertifiedKeyHolder::default(),
        x509_context_rx,
    };

    x509_context_tx
        .send(X509Context::new(vec![], X509BundleSet::new()))
        .expect("the service receives the context");

    let error = Box::new(service)
        .run(CancellationToken::new())
        .await
        .expect_err("the service stops on a context it cannot convert");

    assert!(matches!(
        error.downcast_ref::<SvidError>(),
        Some(SvidError::MissingDefaultSvid)
    ));
    drop(x509_context_tx);
}
