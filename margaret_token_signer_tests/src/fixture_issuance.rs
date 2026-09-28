use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_token_issuance::token_issuance::TokenIssuance;
use margaret_token_trust::token_trust::TokenTrust;

#[must_use]
pub fn fixture_issuance() -> TokenIssuance {
    let TokenTrust { audience, issuer } = fixture_trust();

    TokenIssuance { audience, issuer }
}
