use spiffe::TrustDomain;
use spiffe::X509Bundle;
use spiffe::X509BundleSet;
use spiffe::X509Context;
use spiffe::X509Svid;
use tokio::sync::broadcast;

use margaret_spiffe_svid_manager_tests::build_converter_service::build_converter_service;
use margaret_spiffe_svid_manager_tests::ca_der::CA_DER;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_workload_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;
use margaret_spiffe_svid_manager_tests::leaf_x25519_unsupported_signing_key_der::LEAF_X25519_UNSUPPORTED_SIGNING_KEY_DER;

#[tokio::test]
async fn convert_x509_context_errors_when_private_key_is_unsupported_for_signing() {
    let (ca_bundle_tx, _ca_bundle_rx) = broadcast::channel(1);
    let service = build_converter_service(ca_bundle_tx);

    let cert_chain_der = [LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER, CA_DER].concat();
    let svid =
        X509Svid::parse_from_der(&cert_chain_der, LEAF_X25519_UNSUPPORTED_SIGNING_KEY_DER).unwrap();
    let trust_domain = TrustDomain::new("example.org").unwrap();
    let bundle = X509Bundle::from_x509_authorities(trust_domain, &[CA_DER]).unwrap();
    let mut bundle_set = X509BundleSet::new();

    bundle_set.add_bundle(bundle);

    let context = X509Context::new(vec![svid], bundle_set);

    let result = service.convert_x509_context(context).await;

    assert!(result.is_err());
}
