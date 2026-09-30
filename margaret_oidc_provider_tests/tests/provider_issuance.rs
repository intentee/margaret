use margaret_token_issuance::token_issuance::TokenIssuance;

/// # Panics
///
/// Panics when the fixture issuer identifier or audience is rejected.
#[must_use]
pub fn provider_issuance() -> TokenIssuance {
    TokenIssuance {
        audience: "session"
            .parse()
            .expect("the session audience is not empty"),
        issuer: "https://localhost"
            .parse()
            .expect("the provider issuer is an https url"),
    }
}
