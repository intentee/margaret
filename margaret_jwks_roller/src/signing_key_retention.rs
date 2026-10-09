use std::time::Duration;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::id_token_lifetime_secs::ID_TOKEN_LIFETIME_SECS;
use margaret_identity_session::sign_in_transaction_lifetime_secs::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_issuer_request::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use margaret_jwks_keygen::key_retention::KeyRetention;

use crate::jwks_roll_interval::JWKS_ROLL_INTERVAL;

fn lifetime(seconds: u32) -> Duration {
    Duration::from_secs(u64::from(seconds))
}

#[must_use]
pub fn signing_key_retention() -> KeyRetention {
    let longest_token_lifetime = [
        lifetime(ACCESS_TOKEN_LIFETIME_SECS),
        lifetime(ID_TOKEN_LIFETIME_SECS),
        lifetime(SIGN_IN_TRANSACTION_LIFETIME_SECS),
        ISSUER_REQUEST_TIMEOUT,
    ]
    .into_iter()
    .fold(Duration::ZERO, Duration::max);

    KeyRetention {
        token: longest_token_lifetime.saturating_add(JWKS_ROLL_INTERVAL),
    }
}
