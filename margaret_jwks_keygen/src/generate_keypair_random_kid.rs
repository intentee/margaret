use uuid::Uuid;

use crate::curve::Curve;
use crate::generate_keypair::generate_keypair;
use crate::generate_keypair_params::GenerateKeypairParams;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;

/// # Errors
///
/// Returns `JwksKeyError` propagated from the work it performs.
pub fn generate_keypair_random_kid(crv: Curve) -> Result<JwkPair, JwksKeyError> {
    generate_keypair(GenerateKeypairParams {
        crv,
        kid: Uuid::new_v4().to_string(),
    })
}
