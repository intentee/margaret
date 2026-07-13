use std::sync::Arc;

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid_manager::ca_bundle::CaBundle;
use margaret_spiffe_svid_manager::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_manager::root_cert_store_service::RootCertStoreService;

#[tokio::test]
async fn run_propagates_error_when_channel_closes_without_cancellation() {
    let (ca_bundle_tx, ca_bundle_rx) = broadcast::channel::<Arc<CaBundle<'static>>>(1);
    drop(ca_bundle_tx);

    let service = RootCertStoreService {
        ca_bundle_rx,
        root_cert_store_holder: RootCertStoreHolder::default(),
    };

    let result = Box::new(service).run(CancellationToken::new()).await;

    assert!(result.is_err());
}
