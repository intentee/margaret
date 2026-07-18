use margaret_jwks_client::jwks_poll_interval::JWKS_POLL_INTERVAL;
use margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL;

#[test]
fn jwks_poll_interval_is_shorter_than_the_roll_interval() {
    assert!(JWKS_POLL_INTERVAL < JWKS_ROLL_INTERVAL);
}
