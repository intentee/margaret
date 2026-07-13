use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid_manager_tests::build_converter_service::build_converter_service;

#[tokio::test]
async fn run_propagates_error_when_channel_closes_without_cancellation() {
    let (ca_bundle_tx, _ca_bundle_rx) = broadcast::channel(1);
    let service = build_converter_service(ca_bundle_tx);

    let result = Box::new(service).run(CancellationToken::new()).await;

    assert!(result.is_err());
}
