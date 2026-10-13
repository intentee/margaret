use margaret_jwt_verification_tests::constants::FIXTURE_ISSUER;
use margaret_token_issuance::token_issuance::TokenIssuance;

#[must_use]
pub fn fixture_issuance() -> TokenIssuance {
    TokenIssuance {
        issuer: FIXTURE_ISSUER,
    }
}
