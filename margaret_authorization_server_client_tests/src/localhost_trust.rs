use margaret_token_trust::token_trust::TokenTrust;

/// # Panics
///
/// Panics when the fixture issuer identifier or audience is rejected.
#[must_use]
pub fn localhost_trust() -> TokenTrust {
    TokenTrust {
        audience: "margaret",
        issuer: "https://localhost",
    }
}
