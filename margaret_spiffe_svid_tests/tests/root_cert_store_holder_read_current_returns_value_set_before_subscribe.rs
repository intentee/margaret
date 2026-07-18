use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[tokio::test]
async fn read_current_returns_value_set_before_subscribe() {
    let holder = RootCertStoreHolder::default();
    holder.set(Some(build_root_cert_store_with_ca()));

    let mut subscription = holder.subscribe();

    assert!(subscription.read_current().is_some());
}
