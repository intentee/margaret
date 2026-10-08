use std::sync::Arc;

use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

pub struct JwksRollerServerBundleParams {
    pub rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
    pub storage: Arc<dyn StoresSigningKeys>,
}
