use margaret_token_trust::token_trust::TokenTrust;

/// # Panics
///
/// Panics when the fixture issuer identifier or audience is rejected.
#[must_use]
pub fn localhost_trust() -> TokenTrust {
    TokenTrust {
        audience: "margaret"
            .parse()
            .expect("the fixture audience is not empty"),
        issuer: "https://localhost"
            .parse()
            .expect("the fixture issuer is an https url"),
    }
}
