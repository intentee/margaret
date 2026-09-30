use std::ops::ControlFlow;

use serde::de::DeserializeOwned;

use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::attribute_serialized_jwt::attribute_serialized_jwt;
use crate::jwt_expectation::JwtExpectation;
use crate::jwt_profile::JwtProfile;
use crate::jwt_profiling::JwtProfiling;
use crate::jwt_rejection::JwtRejection;
use crate::jwt_verification::JwtVerification;

#[must_use]
pub fn verify_serialized_jwt<TClaims: DeserializeOwned, TProfile: JwtProfile>(
    key_set: &VerificationKeySet,
    token: &str,
    JwtExpectation { audience, issuer }: &JwtExpectation,
    now: NumericDate,
) -> JwtVerification<TClaims, TProfile> {
    let attributed = match attribute_serialized_jwt(token, issuer) {
        ControlFlow::Continue(attributed) => attributed,
        ControlFlow::Break(rejection) => return JwtVerification::Rejected(rejection),
    };

    match attributed.profile::<TProfile>() {
        JwtProfiling::Profiled(profiled) => profiled.verify(key_set, audience, now),
        JwtProfiling::Rejected(rejection) => {
            JwtVerification::Rejected(JwtRejection::Type(rejection))
        }
    }
}
