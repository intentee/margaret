use std::sync::Arc;

use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid_tests::build_workload_credentials::build_workload_credentials;

#[tokio::test]
async fn returns_key_after_set() {
    let holder = SvidCertifiedKeyHolder::default();
    holder.set(Some(Arc::new(build_workload_credentials())));

    assert!(holder.get().is_some());
}
