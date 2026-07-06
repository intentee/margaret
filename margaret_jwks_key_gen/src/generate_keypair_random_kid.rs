use uuid::Uuid;

use crate::curve::Curve;
use crate::generate_keypair::generate_keypair;
use crate::generate_keypair_params::GenerateKeypairParams;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;

pub fn generate_keypair_random_kid(crv: Curve) -> Result<JwkPair, JwksKeyError> {
    generate_keypair(GenerateKeypairParams {
        crv,
        kid: Uuid::new_v4().to_string(),
    })
}
