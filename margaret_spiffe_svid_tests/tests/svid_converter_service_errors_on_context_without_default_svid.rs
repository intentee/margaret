use spiffe::X509BundleSet;
use spiffe::X509Context;
use tokio::sync::broadcast;

use margaret_spiffe_svid_tests::build_converter_service::build_converter_service;

#[test]
fn errors_on_a_context_without_a_default_svid_since_there_is_no_key_to_serve() {
    let (ca_bundle_tx, _ca_bundle_rx) = broadcast::channel(1);
    let service = build_converter_service(ca_bundle_tx);

    let result = service.convert_x509_context(&X509Context::new(vec![], X509BundleSet::new()));

    assert!(result.is_err());
}
