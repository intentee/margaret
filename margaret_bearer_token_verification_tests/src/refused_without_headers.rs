use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;

#[must_use]
pub fn refused_without_headers<TToken>(admission: &TokenAdmission<TToken>, status: u16) -> bool {
    matches!(
        admission,
        TokenAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == status && response.headers().is_empty()
    )
}
