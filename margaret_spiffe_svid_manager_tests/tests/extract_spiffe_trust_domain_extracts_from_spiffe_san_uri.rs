use margaret_spiffe_svid_manager::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;

#[test]
fn extracts_trust_domain_from_spiffe_san_uri() {
    let result = extract_spiffe_trust_domain(LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER);

    assert_eq!(result.unwrap(), "example.org");
}
