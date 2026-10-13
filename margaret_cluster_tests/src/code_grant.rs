use margaret::framework::oauth_vocabulary::grant_type::GrantType;
use margaret_oidc_provider_tests::constants::PKCE_VERIFIER;

use crate::constants::PARTNER_REDIRECT_URI;

#[must_use]
pub fn code_grant(code: &str) -> [[&str; 2]; 4] {
    [
        ["code", code],
        ["code_verifier", PKCE_VERIFIER],
        ["grant_type", GrantType::AuthorizationCode.wire_name()],
        ["redirect_uri", PARTNER_REDIRECT_URI],
    ]
}
