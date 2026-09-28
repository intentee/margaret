use std::time::Duration;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL;

#[test]
fn jwks_roll_interval_outlasts_the_access_token_lifetime() {
    let access_token_lifetime = Duration::from_secs(u64::from(ACCESS_TOKEN_LIFETIME_SECS));

    assert!(JWKS_ROLL_INTERVAL >= access_token_lifetime);
}
