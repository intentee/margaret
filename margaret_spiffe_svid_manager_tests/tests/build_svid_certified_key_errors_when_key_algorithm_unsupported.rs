use margaret_spiffe_svid_manager::build_svid_certified_key::build_svid_certified_key;
use margaret_spiffe_svid_manager_tests::build_workload_cert_chain::build_workload_cert_chain;
use margaret_spiffe_svid_manager_tests::leaf_x25519_unsupported_signing_key_der::LEAF_X25519_UNSUPPORTED_SIGNING_KEY_DER;

#[test]
fn errors_when_private_key_algorithm_is_not_supported_for_signing() {
    let result = build_svid_certified_key(
        build_workload_cert_chain(),
        LEAF_X25519_UNSUPPORTED_SIGNING_KEY_DER,
    );

    assert!(result.is_err());
}
