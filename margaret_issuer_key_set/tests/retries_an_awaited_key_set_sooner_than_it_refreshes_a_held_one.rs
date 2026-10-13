use margaret_issuer_key_set::constants::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::constants::KEY_SET_POLL_INTERVAL_AFTER_READY;

#[test]
fn retries_an_awaited_key_set_sooner_than_it_refreshes_a_held_one() {
    assert!(ISSUER_FETCH_SPACING < KEY_SET_POLL_INTERVAL_AFTER_READY);
}
