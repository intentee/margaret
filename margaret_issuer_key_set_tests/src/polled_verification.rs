use serde_json::Value;

use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;

use crate::refresh_observation::RefreshObservation;

pub struct PolledVerification {
    pub refresh: RefreshObservation,
    pub verification: IssuerVerification<Value, AccessTokenProfile>,
}
