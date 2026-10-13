use margaret_spiffe_svid::build_svid_certified_key::build_svid_certified_key;
use margaret_spiffe_svid_tests::build_workload_cert_chain::build_workload_cert_chain;
use margaret_spiffe_svid_tests::constants::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER;

#[test]
fn builds_a_certified_key_for_valid_inputs_so_the_server_can_present_its_svid() {
    let result = build_svid_certified_key(
        build_workload_cert_chain(),
        LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
    );

    assert!(result.is_ok());
}
