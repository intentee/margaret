use margaret_issuer_key_set::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;

#[test]
fn polls_a_held_key_set_more_often_than_margaret_rolls_its_keys() {
    assert!(KEY_SET_POLL_INTERVAL_AFTER_READY < JWKS_ROLL_INTERVAL);
}
