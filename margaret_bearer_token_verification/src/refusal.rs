use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::bearer_token_admission::BearerTokenAdmission;

pub(crate) fn refusal<TClaims, TProfile>(
    response: Response,
) -> BearerTokenAdmission<TClaims, TProfile> {
    BearerTokenAdmission::Refused(ResponseContinuation::from(response))
}
