use margaret_token_trust::token_trust::TokenTrust;

#[must_use]
pub fn localhost_trust() -> TokenTrust {
    TokenTrust {
        audience: "margaret",
        issuer: "https://localhost",
    }
}
