use std::sync::Arc;

use margaret_issuer_key_set::held_key_set::HeldKeySet;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

#[tokio::test]
async fn waits_for_a_fetch_that_starts_after_the_snapshot() {
    let issuer_key_set = IssuerKeySet::awaiting();

    issuer_key_set.start_fetch();

    let snapshot = issuer_key_set.snapshot();
    let (refresh, ()) = tokio::join!(issuer_key_set.refreshed_since(&snapshot), async {
        issuer_key_set.hold(Arc::new(fresh_p256_secret().key_set().clone()));
        issuer_key_set.refresh_requested().await;
        issuer_key_set.start_fetch();
        issuer_key_set.fail_fetch();
        issuer_key_set.start_fetch();
        issuer_key_set.hold(Arc::new(fresh_p256_secret().key_set().clone()));
    });
    let KeySetRefresh::Refreshed(KeySetHolding::Held(HeldKeySet {
        key_set: refreshed, ..
    })) = refresh
    else {
        panic!("the refresh holds a key set");
    };
    let KeySetHolding::Held(HeldKeySet {
        key_set: latest, ..
    }) = issuer_key_set.snapshot().holding
    else {
        panic!("the latest fetch holds a key set");
    };

    assert!(Arc::ptr_eq(&refreshed, &latest));
}
