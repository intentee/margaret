use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;

#[test]
fn returns_none_before_set() {
    let holder = RootCertStoreHolder::default();

    assert!(holder.get().is_none());
}
