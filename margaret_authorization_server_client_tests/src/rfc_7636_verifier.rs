use margaret_oauth_vocabulary::code_verifier::CodeVerifier;

/// # Panics
///
/// Panics when the example verifier of RFC 7636 appendix B is rejected.
#[must_use]
pub fn rfc_7636_verifier() -> CodeVerifier {
    serde_json::from_value(serde_json::Value::String(
        "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk".to_string(),
    ))
    .expect("the rfc 7636 example verifier is accepted")
}
