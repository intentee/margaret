use oauth2::CsrfToken;

const RANDOM_TOKEN_OCTETS: u32 = 32;

#[must_use]
pub fn random_token() -> String {
    CsrfToken::new_random_len(RANDOM_TOKEN_OCTETS).into_secret()
}
