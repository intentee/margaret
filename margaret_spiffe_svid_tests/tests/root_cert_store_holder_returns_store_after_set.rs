use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn returns_store_after_set() {
    let holder = RootCertStoreHolder::default();
    holder.set(Some(build_root_cert_store_with_ca()));

    assert!(holder.get().is_some());
}
