use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;

#[tokio::test]
async fn reports_polling_that_stopped_before_the_refetch() {
    let issuer_key_set = IssuerKeySet::awaiting();
    let snapshot = issuer_key_set.snapshot();

    issuer_key_set.stop_polling();

    assert!(matches!(
        issuer_key_set.request_refresh_after(&snapshot).await,
        KeySetRefresh::PollingStopped
    ));
}
