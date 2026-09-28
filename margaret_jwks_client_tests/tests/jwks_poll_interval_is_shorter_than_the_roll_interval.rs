use margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_key_set_poll::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;

#[test]
fn jwks_poll_interval_is_shorter_than_the_roll_interval() {
    assert!(KEY_SET_POLL_INTERVAL_AFTER_READY < JWKS_ROLL_INTERVAL);
}
