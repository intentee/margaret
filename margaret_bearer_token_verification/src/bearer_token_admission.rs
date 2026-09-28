use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum BearerTokenAdmission<TClaims> {
    Anonymous,
    Presented(VerifiedJwt<TClaims>),
    Refused(ResponseContinuation),
}
