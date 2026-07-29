use spiffe::spiffe_id::TrustDomain;

use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_workload_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;

#[test]
fn extracts_trust_domain_from_spiffe_san_uri() {
    let result = extract_spiffe_trust_domain(LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER);

    assert_eq!(
        result.expect("the certificate carries a spiffe uri san"),
        TrustDomain::new("example.org").expect("the trust domain is valid")
    );
}
