use serde::de::DeserializeOwned;

use margaret_issuer_key_set::held_key_set::HeldKeySet;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::profiled_jwt::ProfiledJwt;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::issuer_verification::IssuerVerification;
use crate::key_set_verification::KeySetVerification;

pub(crate) fn verify_with_key_set<TClaims: DeserializeOwned, TProfile: JwtProfile>(
    jwt: &ProfiledJwt<'_, TProfile>,
    holding: &KeySetHolding,
    audience: &Audience,
    now: NumericDate,
) -> KeySetVerification<TClaims, TProfile> {
    let KeySetHolding::Held(HeldKeySet {
        fetched_at,
        key_set,
    }) = holding
    else {
        return KeySetVerification::Settled(IssuerVerification::KeysAwaited);
    };

    match jwt.verify(key_set, &ExpectedAudience::One(audience), now) {
        JwtVerification::Rejected(
            rejection @ JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }),
        ) => KeySetVerification::UnknownKey {
            fetched_at: *fetched_at,
            rejection,
        },
        JwtVerification::Rejected(rejection) => {
            KeySetVerification::Settled(IssuerVerification::Rejected(rejection))
        }
        JwtVerification::Verified(verified) => {
            KeySetVerification::Settled(IssuerVerification::Verified(verified))
        }
    }
}
