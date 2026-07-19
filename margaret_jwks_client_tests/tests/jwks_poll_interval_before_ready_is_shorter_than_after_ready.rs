use margaret_jwks_client::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_client::jwks_poll_interval_before_ready::JWKS_POLL_INTERVAL_BEFORE_READY;

#[test]
fn jwks_poll_interval_before_ready_is_shorter_than_after_ready() {
    assert!(JWKS_POLL_INTERVAL_BEFORE_READY < JWKS_POLL_INTERVAL_AFTER_READY);
}
