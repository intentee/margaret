use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::jwks_roll_interval::JWKS_ROLL_INTERVAL;

#[must_use]
pub fn roll_due_at(secret: &JwksSecret) -> NumericDate {
    secret.rolled_at().after(JWKS_ROLL_INTERVAL)
}
