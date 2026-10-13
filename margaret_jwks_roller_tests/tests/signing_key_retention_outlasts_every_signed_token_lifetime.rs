use std::time::Duration;

use margaret_identity_session::constants::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::constants::ID_TOKEN_LIFETIME_SECS;
use margaret_identity_session::constants::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_issuer_request::constants::ISSUER_REQUEST_TIMEOUT;
use margaret_jwks_roller::constants::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;

#[test]
fn signing_key_retention_outlasts_every_signed_token_lifetime() {
    let retention = signing_key_retention();

    for lifetime in [
        Duration::from_secs(u64::from(ACCESS_TOKEN_LIFETIME_SECS)),
        Duration::from_secs(u64::from(ID_TOKEN_LIFETIME_SECS)),
        Duration::from_secs(u64::from(SIGN_IN_TRANSACTION_LIFETIME_SECS)),
        ISSUER_REQUEST_TIMEOUT,
    ] {
        assert!(retention.token >= lifetime + JWKS_ROLL_INTERVAL);
    }
}
