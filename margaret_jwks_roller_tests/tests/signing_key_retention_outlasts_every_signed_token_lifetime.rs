use std::time::Duration;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::id_token_lifetime_secs::ID_TOKEN_LIFETIME_SECS;
use margaret_identity_session::refresh_token_lifetime_secs::REFRESH_TOKEN_LIFETIME_SECS;
use margaret_identity_session::sign_in_transaction_lifetime_secs::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_issuer_request::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
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

    assert!(
        retention.refresh
            >= Duration::from_secs(u64::from(REFRESH_TOKEN_LIFETIME_SECS)) + JWKS_ROLL_INTERVAL
    );
}
