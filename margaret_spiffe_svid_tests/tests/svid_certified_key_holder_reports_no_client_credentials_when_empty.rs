use rustls::client::ResolvesClientCert as _;

use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;

#[test]
fn reports_no_client_credentials_while_empty_so_no_bogus_cert_is_presented() {
    let holder = SvidCertifiedKeyHolder::default();

    assert!(!holder.has_certs());
    assert!(holder.resolve(&[], &[]).is_none());
}
