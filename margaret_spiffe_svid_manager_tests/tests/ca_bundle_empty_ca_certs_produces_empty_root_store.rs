use margaret_spiffe_svid_manager::ca_bundle::CaBundle;

#[test]
fn empty_ca_certs_produces_empty_root_store() {
    let bundle = CaBundle { ca_certs: vec![] };

    let result = bundle.to_root_cert_store();

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
