use margaret_spiffe_svid_manager_tests::build_converter_service::build_converter_service;
use margaret_spiffe_svid_manager_tests::x509_context_builder::build_workload_x509_context;
use tokio::sync::broadcast;

#[tokio::test]
async fn convert_x509_context_errors_when_ca_bundle_channel_has_no_subscribers() {
    let (ca_bundle_tx, ca_bundle_rx) = broadcast::channel(1);

    drop(ca_bundle_rx);

    let service = build_converter_service(ca_bundle_tx);
    let context = build_workload_x509_context().unwrap();

    let result = service.convert_x509_context(context).await;

    assert!(result.is_err());
}
