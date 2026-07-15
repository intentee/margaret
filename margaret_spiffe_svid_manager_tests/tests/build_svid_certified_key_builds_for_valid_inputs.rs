use margaret_spiffe_svid_manager::build_svid_certified_key::build_svid_certified_key;
use margaret_spiffe_svid_manager_tests::build_workload_cert_chain::build_workload_cert_chain;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_workload_key_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER;

#[test]
fn builds_certified_key_for_valid_inputs() {
    let result = build_svid_certified_key(
        build_workload_cert_chain(),
        LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
    );

    assert!(result.is_ok());
}
