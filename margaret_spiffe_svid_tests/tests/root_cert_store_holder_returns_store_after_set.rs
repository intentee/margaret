use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn returns_the_root_store_once_set_so_verifiers_can_trust_the_rotated_bundle() {
    let holder = RootCertStoreHolder::default();
    holder.set(Some(build_root_cert_store_with_ca()));

    assert!(holder.get().is_some());
}
