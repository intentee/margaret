use margaret_key_set_poll::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;

#[test]
fn key_set_poll_interval_before_ready_is_shorter_than_after_ready() {
    assert!(KEY_SET_POLL_INTERVAL_BEFORE_READY < KEY_SET_POLL_INTERVAL_AFTER_READY);
}
