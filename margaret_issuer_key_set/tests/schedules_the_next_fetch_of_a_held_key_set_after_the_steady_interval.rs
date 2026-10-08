use std::sync::Arc;

use tokio::time::Instant;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;

#[test]
fn schedules_the_next_fetch_of_a_held_key_set_after_the_steady_interval() {
    let issuer_key_set = IssuerKeySet::awaiting();
    let fetch_started_at = Instant::now();

    issuer_key_set.start_fetch();
    issuer_key_set.hold(Arc::new(
        fresh_secret(SigningCurve::P256).published_key_set().clone(),
    ));

    assert_eq!(
        issuer_key_set
            .snapshot()
            .holding
            .next_fetch_due(fetch_started_at),
        fetch_started_at + KEY_SET_POLL_INTERVAL_AFTER_READY
    );
}
