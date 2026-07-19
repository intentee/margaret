use margaret_jwks_client::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL;

#[test]
fn jwks_poll_interval_is_shorter_than_the_roll_interval() {
    assert!(JWKS_POLL_INTERVAL_AFTER_READY < JWKS_ROLL_INTERVAL);
}
