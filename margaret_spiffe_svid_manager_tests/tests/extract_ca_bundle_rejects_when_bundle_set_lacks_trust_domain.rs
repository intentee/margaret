use spiffe::X509BundleSet;
use spiffe::X509Context;

use margaret_spiffe_svid_manager::extract_ca_bundle::extract_ca_bundle;
use margaret_spiffe_svid_manager_tests::build_workload_svid::build_workload_svid;

#[test]
fn rejects_when_bundle_set_lacks_trust_domain() {
    let svid = build_workload_svid();
    let context = X509Context::new(vec![svid.clone()], X509BundleSet::new());

    let result = extract_ca_bundle(&svid, &context);

    assert!(result.is_err());
}
