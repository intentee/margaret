use std::ops::ControlFlow;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification::verified_jws::VerifiedJws;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::claims_rejection::ClaimsRejection;
use crate::jwt_expectation::JwtExpectation;
use crate::jwt_rejection::JwtRejection;
use crate::jwt_verification::JwtVerification;
use crate::verified_jwt::VerifiedJwt;

#[derive(Deserialize)]
struct JwtPayload<TClaims> {
    #[serde(flatten)]
    registered: RegisteredClaims,
    #[serde(flatten)]
    claims: TClaims,
}

#[must_use]
pub fn verify_jwt<TClaims: DeserializeOwned>(
    key_set: &VerificationKeySet,
    token: &str,
    JwtExpectation {
        audience,
        issuer,
        token_type,
    }: &JwtExpectation,
    now: NumericDate,
) -> JwtVerification<TClaims> {
    let VerifiedJws { kid, payload, typ } = match key_set.verify(token) {
        JwsVerification::Rejected(rejection) => {
            return JwtVerification::Rejected(JwtRejection::Jws(rejection));
        }
        JwsVerification::Verified(verified) => verified,
    };

    if let ControlFlow::Break(rejection) = token_type.check(typ) {
        return JwtVerification::Rejected(JwtRejection::Type(rejection));
    }

    let JwtPayload { registered, claims } = match serde_json::from_slice(&payload) {
        Ok(payload) => payload,
        Err(source) => {
            return JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Malformed {
                source,
            }));
        }
    };

    if registered.iss != issuer.as_str() {
        return JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::IssuerMismatch {
            expected: (*issuer).clone(),
            found: registered.iss,
        }));
    }

    if !registered.aud.is_exactly(audience) {
        return JwtVerification::Rejected(JwtRejection::Claims(
            ClaimsRejection::AudienceMismatch {
                expected: (*audience).clone(),
                found: registered.aud,
            },
        ));
    }

    if now >= registered.exp {
        return JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Expired {
            exp: registered.exp,
            now,
        }));
    }

    if let Some(nbf) = registered.nbf
        && now < nbf
    {
        return JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::NotYetValid {
            nbf,
            now,
        }));
    }

    JwtVerification::Verified(VerifiedJwt {
        claims,
        kid,
        registered,
    })
}
