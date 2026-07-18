use spiffe::X509BundleSet;
use spiffe::X509Context;
use tokio::sync::broadcast;

use margaret_spiffe_svid_tests::build_converter_service::build_converter_service;
use margaret_spiffe_svid_tests::build_workload_svid::build_workload_svid;

#[tokio::test]
async fn convert_x509_context_errors_when_bundle_lacks_trust_domain() {
    let (ca_bundle_tx, _ca_bundle_rx) = broadcast::channel(1);
    let service = build_converter_service(ca_bundle_tx);

    let context = X509Context::new(vec![build_workload_svid()], X509BundleSet::new());

    let result = service.convert_x509_context(context).await;

    assert!(result.is_err());
}
