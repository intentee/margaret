use std::time::SystemTime;

use serde::de::DeserializeOwned;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

use crate::bearer_token_admission::BearerTokenAdmission;
use crate::bearer_token_verification::BearerTokenVerification;
use crate::bearer_token_verifier::BearerTokenVerifier;

fn refusal<TClaims>(response: Response) -> BearerTokenAdmission<TClaims> {
    BearerTokenAdmission::Refused(ResponseContinuation::from(response))
}

/// # Errors
///
/// Returns `RegisteredClaimsError` when the system clock cannot be read as a numeric date.
pub fn admit_bearer_token<TClaims: DeserializeOwned>(
    verifier: &BearerTokenVerifier,
    authorization: &RequestAuthorization,
) -> Result<BearerTokenAdmission<TClaims>, RegisteredClaimsError> {
    NumericDate::from_system_time(SystemTime::now()).map(|now| {
        match verifier.verify(authorization, now) {
            BearerTokenVerification::Absent | BearerTokenVerification::NotBearer => {
                BearerTokenAdmission::Anonymous
            }
            BearerTokenVerification::MalformedAuthorization => {
                refusal(BearerChallenge::InvalidRequest.response())
            }
            BearerTokenVerification::Rejected(_) => {
                refusal(BearerChallenge::InvalidToken.response())
            }
            BearerTokenVerification::Unavailable => refusal(Response::text(
                503,
                "The signing keys of the token issuer are not available yet",
            )),
            BearerTokenVerification::Verified(token) => BearerTokenAdmission::Presented(token),
        }
    })
}
