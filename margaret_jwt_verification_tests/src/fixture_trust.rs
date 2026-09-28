use margaret_token_trust::token_trust::TokenTrust;

#[must_use]
/// # Panics
///
/// Panics when the fixture issuer identifier or audience is rejected.
pub fn fixture_trust() -> TokenTrust {
    TokenTrust {
        audience: "margaret"
            .parse()
            .expect("the fixture audience is not empty"),
        issuer: "https://issuer.example"
            .parse()
            .expect("the fixture issuer is an https url"),
    }
}
