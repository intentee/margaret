use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;

#[test]
fn retries_an_awaited_key_set_sooner_than_it_refreshes_a_held_one() {
    assert!(ISSUER_FETCH_SPACING < KEY_SET_POLL_INTERVAL_AFTER_READY);
}
