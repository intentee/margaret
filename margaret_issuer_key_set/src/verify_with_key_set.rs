use serde::de::DeserializeOwned;

use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::key_selection::KeySelection;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::profiled_jwt::ProfiledJwt;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::held_key_set::HeldKeySet;
use crate::issuer_verification::IssuerVerification;
use crate::key_set_holding::KeySetHolding;
use crate::key_set_verification::KeySetVerification;

pub(crate) fn verify_with_key_set<TClaims: DeserializeOwned, TProfile: JwtProfile>(
    jwt: &ProfiledJwt<'_, TProfile>,
    holding: &KeySetHolding,
    now: NumericDate,
) -> KeySetVerification<TClaims, TProfile> {
    let KeySetHolding::Held(HeldKeySet {
        fetched_at,
        key_set,
    }) = holding
    else {
        return KeySetVerification::Settled(IssuerVerification::KeysAwaited);
    };

    match jwt.verify(key_set, now) {
        JwtVerification::Rejected(
            rejection @ JwtRejection::Jws(
                JwsRejection::NoKeyForAlgorithm { .. }
                | JwsRejection::SignatureMismatch {
                    selection: KeySelection::SoleKeyOfAlgorithm,
                    ..
                }
                | JwsRejection::UnknownKeyId { .. },
            ),
        ) => KeySetVerification::KeysPossiblyRotated {
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
