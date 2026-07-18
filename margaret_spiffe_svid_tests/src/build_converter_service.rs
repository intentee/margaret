use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

use margaret_spiffe_svid::ca_bundle::CaBundle;
use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid::svid_converter_service::SvidConverterService;

#[must_use]
pub fn build_converter_service(
    ca_bundle_tx: Sender<Arc<CaBundle<'static>>>,
) -> SvidConverterService {
    let (_x509_context_tx, x509_context_rx) = broadcast::channel(1);

    SvidConverterService {
        ca_bundle_tx,
        svid_certified_key_holder: SvidCertifiedKeyHolder::default(),
        x509_context_rx,
    }
}
