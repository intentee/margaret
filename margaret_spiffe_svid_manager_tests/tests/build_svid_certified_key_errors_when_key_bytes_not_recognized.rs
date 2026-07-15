use margaret_spiffe_svid_manager::build_svid_certified_key::build_svid_certified_key;
use margaret_spiffe_svid_manager_tests::build_workload_cert_chain::build_workload_cert_chain;

#[test]
fn errors_when_private_key_bytes_are_not_a_recognized_format() {
    let result = build_svid_certified_key(build_workload_cert_chain(), &[0u8; 8]);

    assert!(result.is_err());
}
