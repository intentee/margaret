use rustls::SignatureScheme;

use margaret_spiffe_svid::extract_server_credentials::extract_server_credentials;
use margaret_spiffe_svid_tests::build_workload_svid::build_workload_svid;

#[test]
fn produces_valid_certified_key() {
    let credentials = extract_server_credentials(&build_workload_svid()).unwrap();

    assert_eq!(credentials.cert_chain.len(), 2);
    assert!(
        credentials
            .certified_key
            .key
            .choose_scheme(&[SignatureScheme::ECDSA_NISTP256_SHA256])
            .is_some()
    );
}
