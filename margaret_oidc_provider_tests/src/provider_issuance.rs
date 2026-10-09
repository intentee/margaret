use margaret_token_issuance::token_issuance::TokenIssuance;

#[must_use]
pub fn provider_issuance() -> TokenIssuance {
    TokenIssuance {
        issuer: "https://localhost",
    }
}
