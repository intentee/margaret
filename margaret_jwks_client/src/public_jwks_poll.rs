use margaret_jws_verification::verification_key_set::VerificationKeySet;

use crate::jwks_client_error::JwksClientError;

pub enum PublicJwksPoll {
    Cancelled,
    Failed(JwksClientError),
    Fetched(VerificationKeySet),
}
