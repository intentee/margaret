use std::sync::Arc;

use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

pub struct JwksRollerServerBundleParams {
    pub rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
    pub storage: Arc<dyn JwksSecretStorage>,
}
