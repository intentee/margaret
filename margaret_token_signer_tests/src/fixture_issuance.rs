use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_token_issuance::token_issuance::TokenIssuance;

#[must_use]
pub fn fixture_issuance() -> TokenIssuance {
    TokenIssuance {
        audience: FIXTURE_AUDIENCE,
        issuer: FIXTURE_ISSUER,
    }
}
