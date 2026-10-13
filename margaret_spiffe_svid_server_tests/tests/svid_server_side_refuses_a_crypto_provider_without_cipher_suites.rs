use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;
use margaret_spiffe_svid_server::svid_error::SvidError;
use margaret_spiffe_svid_server::svid_server_side::SvidServerSide;
use margaret_spiffe_svid_tests::suiteless_crypto_provider::suiteless_crypto_provider;

#[test]
fn refuses_a_crypto_provider_without_cipher_suites() {
    assert!(matches!(
        SvidServerSide::new(SvidSideParams {
            crypto_provider: suiteless_crypto_provider(),
            root_cert_store_holder: RootCertStoreHolder::default(),
            spiffe_trust_domain: "example.org".to_string(),
            svid_certified_key_holder: SvidCertifiedKeyHolder::default(),
        }),
        Err(SvidError::ProtocolVersions { .. })
    ));
}
