use margaret_token_trust::token_trust::TokenTrust;

use crate::fixture_audience::FIXTURE_AUDIENCE;
use crate::fixture_issuer::FIXTURE_ISSUER;

#[must_use]
pub fn fixture_trust() -> TokenTrust {
    TokenTrust {
        audience: FIXTURE_AUDIENCE,
        issuer: FIXTURE_ISSUER,
    }
}
