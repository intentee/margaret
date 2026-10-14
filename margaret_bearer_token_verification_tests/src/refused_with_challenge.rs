use http::header::WWW_AUTHENTICATE;

use margaret_http::token_admission::TokenAdmission;

#[must_use]
pub fn refused_with_challenge<TToken>(
    admission: &TokenAdmission<TToken>,
    status: u16,
    challenge: &str,
) -> bool {
    matches!(
        admission,
        TokenAdmission::Refused(response)
            if response.status() == status
                && response.header_value(&WWW_AUTHENTICATE) == Some(challenge)
    )
}
