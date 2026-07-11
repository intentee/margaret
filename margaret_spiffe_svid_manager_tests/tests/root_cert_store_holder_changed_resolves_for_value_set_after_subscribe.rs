use margaret_spiffe_svid_manager::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_manager_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[tokio::test]
async fn changed_resolves_for_value_set_after_subscribe() {
    let holder = RootCertStoreHolder::default();

    let mut subscription = holder.subscribe();

    assert!(subscription.read_current().is_none());

    holder.set(Some(build_root_cert_store_with_ca()));

    subscription.changed().await;

    assert!(subscription.read_current().is_some());
}
