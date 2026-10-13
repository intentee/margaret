use rustls::SignatureScheme;

use margaret_spiffe_svid::extract_server_credentials::extract_server_credentials;
use margaret_spiffe_svid_tests::build_workload_svid::build_workload_svid;

#[test]
fn extracts_the_svid_chain_and_an_ecdsa_p256_signing_key_for_the_server_to_present() {
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
