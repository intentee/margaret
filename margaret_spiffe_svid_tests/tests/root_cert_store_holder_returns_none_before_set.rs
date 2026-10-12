use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;

#[test]
fn reports_no_root_store_before_one_is_set_so_trust_is_not_assumed_early() {
    let holder = RootCertStoreHolder::default();

    assert!(holder.get().is_none());
}
