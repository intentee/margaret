use std::marker::PhantomData;
use std::ops::ControlFlow;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification::verified_jws::VerifiedJws;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::attributed_jwt::AttributedJwt;
use crate::claims_rejection::ClaimsRejection;
use crate::expected_audience::ExpectedAudience;
use crate::jwt_profile::JwtProfile;
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

fn rejected<TClaims, TProfile>(rejection: ClaimsRejection) -> JwtVerification<TClaims, TProfile> {
    JwtVerification::Rejected(JwtRejection::Claims(rejection))
}

pub struct ProfiledJwt<'jwt, TProfile> {
    pub(crate) jwt: &'jwt AttributedJwt<'jwt>,
    pub(crate) profile: PhantomData<TProfile>,
}

impl<TProfile: JwtProfile> ProfiledJwt<'_, TProfile> {
    #[must_use]
    pub fn verify<TClaims: DeserializeOwned>(
        &self,
        key_set: &VerificationKeySet,
        expected_audience: &ExpectedAudience<'_>,
        now: NumericDate,
    ) -> JwtVerification<TClaims, TProfile> {
        let VerifiedJws { kid } = match key_set.verify(&self.jwt.jws) {
            JwsVerification::Rejected(rejection) => {
                return JwtVerification::Rejected(JwtRejection::Jws(rejection));
            }
            JwsVerification::Verified(verified) => verified,
        };
        let JwtPayload { registered, claims } = match JwtPayload::deserialize(&self.jwt.payload) {
            Ok(payload) => payload,
            Err(source) => return rejected(ClaimsRejection::Malformed { source }),
        };

        if let ControlFlow::Break(rejection) = expected_audience.admits(&registered.aud) {
            return rejected(rejection);
        }

        if now >= registered.exp {
            return rejected(ClaimsRejection::Expired {
                exp: registered.exp,
                now,
            });
        }

        if let Some(nbf) = registered.nbf
            && now < nbf
        {
            return rejected(ClaimsRejection::NotYetValid { nbf, now });
        }

        JwtVerification::Verified(VerifiedJwt {
            claims,
            kid: kid.clone(),
            registered,
            profile: PhantomData,
        })
    }
}
