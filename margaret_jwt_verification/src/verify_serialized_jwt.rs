use serde::de::DeserializeOwned;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::jwt_expectation::JwtExpectation;
use crate::jwt_rejection::JwtRejection;
use crate::jwt_verification::JwtVerification;
use crate::verify_jwt::verify_jwt;

#[must_use]
pub fn verify_serialized_jwt<TClaims: DeserializeOwned>(
    key_set: &VerificationKeySet,
    token: &str,
    expectation: &JwtExpectation,
    now: NumericDate,
) -> JwtVerification<TClaims> {
    match CompactJws::parse(token) {
        CompactJwsParsing::Parsed(jws) => verify_jwt(key_set, &jws, expectation, now),
        CompactJwsParsing::Rejected(rejection) => {
            JwtVerification::Rejected(JwtRejection::Jws(rejection))
        }
    }
}
