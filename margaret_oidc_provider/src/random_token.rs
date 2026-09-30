use oauth2::CsrfToken;

const RANDOM_TOKEN_OCTETS: u32 = 32;

pub(crate) fn random_token() -> String {
    CsrfToken::new_random_len(RANDOM_TOKEN_OCTETS).into_secret()
}
