use spiffe::X509BundleSet;
use spiffe::X509Context;
use tokio::sync::broadcast;

use margaret_spiffe_svid_tests::build_converter_service::build_converter_service;

#[test]
fn errors_on_context_without_default_svid() {
    let (ca_bundle_tx, _ca_bundle_rx) = broadcast::channel(1);
    let service = build_converter_service(ca_bundle_tx);

    let result = service.convert_x509_context(&X509Context::new(vec![], X509BundleSet::new()));

    assert!(result.is_err());
}
