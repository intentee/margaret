use http::header::WWW_AUTHENTICATE;

use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_http::response_continuation::ResponseContinuation;

#[must_use]
pub fn refused_with_challenge<TClaims, TProfile>(
    admission: &BearerTokenAdmission<TClaims, TProfile>,
    status: u16,
    challenge: &str,
) -> bool {
    matches!(
        admission,
        BearerTokenAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == status
                && response
                    .headers()
                    .iter()
                    .any(|header| header.name == WWW_AUTHENTICATE.as_str() && header.value == challenge)
    )
}
