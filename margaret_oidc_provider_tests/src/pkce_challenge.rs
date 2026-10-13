use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;

use crate::constants::PKCE_VERIFIER;

#[must_use]
pub fn pkce_challenge() -> String {
    PkceCodeChallenge::from_code_verifier_sha256(&PkceCodeVerifier::new(PKCE_VERIFIER.to_string()))
        .as_str()
        .to_string()
}
