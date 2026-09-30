use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_http::response_continuation::ResponseContinuation;

#[must_use]
pub fn refused_without_headers<TClaims, TProfile>(
    admission: &BearerTokenAdmission<TClaims, TProfile>,
    status: u16,
) -> bool {
    matches!(
        admission,
        BearerTokenAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == status && response.headers().is_empty()
    )
}
