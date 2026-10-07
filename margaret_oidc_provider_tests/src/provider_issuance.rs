use margaret_token_issuance::token_issuance::TokenIssuance;

#[must_use]
pub fn provider_issuance() -> TokenIssuance {
    TokenIssuance {
        audience: "session",
        issuer: "https://localhost",
    }
}
