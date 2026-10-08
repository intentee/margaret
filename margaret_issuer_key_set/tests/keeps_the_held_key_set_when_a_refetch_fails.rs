use std::sync::Arc;

use margaret_issuer_key_set::held_key_set::HeldKeySet;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

#[tokio::test]
async fn keeps_the_held_key_set_when_a_refetch_fails() {
    let issuer_key_set = IssuerKeySet::awaiting();

    issuer_key_set.start_fetch();
    issuer_key_set.hold(Arc::new(fresh_p256_secret().key_set().clone()));

    let snapshot = issuer_key_set.snapshot();
    let (refresh, ()) = tokio::join!(issuer_key_set.request_refresh_after(&snapshot), async {
        issuer_key_set.refresh_requested().await;
        issuer_key_set.start_fetch();
        issuer_key_set.fail_fetch();
    });
    let KeySetHolding::Held(HeldKeySet { key_set: kept, .. }) = issuer_key_set.snapshot().holding
    else {
        panic!("the key set is still held");
    };
    let KeySetHolding::Held(HeldKeySet { key_set: held, .. }) = snapshot.holding else {
        panic!("the snapshot holds a key set");
    };

    assert!(matches!(refresh, KeySetRefresh::Unchanged));
    assert!(Arc::ptr_eq(&kept, &held));
}
