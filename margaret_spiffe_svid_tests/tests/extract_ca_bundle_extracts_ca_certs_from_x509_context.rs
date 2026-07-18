use spiffe::TrustDomain;
use spiffe::X509Bundle;
use spiffe::X509BundleSet;
use spiffe::X509Context;

use margaret_spiffe_svid::extract_ca_bundle::extract_ca_bundle;
use margaret_spiffe_svid_tests::build_workload_svid::build_workload_svid;
use margaret_spiffe_svid_tests::ca_der::CA_DER;

#[test]
fn extracts_ca_certs_from_x509_context() {
    let svid = build_workload_svid();
    let trust_domain = TrustDomain::new("example.org").unwrap();
    let bundle = X509Bundle::from_x509_authorities(trust_domain, &[CA_DER]).unwrap();
    let mut bundle_set = X509BundleSet::new();

    bundle_set.add_bundle(bundle);

    let context = X509Context::new(vec![svid.clone()], bundle_set);
    let ca_bundle = extract_ca_bundle(&svid, &context).unwrap();

    assert_eq!(ca_bundle.ca_certs.len(), 1);
}
