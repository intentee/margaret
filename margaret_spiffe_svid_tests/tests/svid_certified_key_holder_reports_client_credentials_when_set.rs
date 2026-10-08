use std::sync::Arc;

use rustls::client::ResolvesClientCert as _;
use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid::build_svid_certified_key::build_svid_certified_key;
use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_client_key_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER;

#[test]
fn reports_client_credentials_when_set() {
    let credentials = build_svid_certified_key(
        vec![CertificateDer::from(
            LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec(),
        )],
        LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_KEY_DER,
    )
    .unwrap();

    let holder = SvidCertifiedKeyHolder::default();
    holder.set(Some(Arc::new(credentials)));

    assert!(holder.has_certs());
    assert!(holder.resolve(&[], &[]).is_some());
}
