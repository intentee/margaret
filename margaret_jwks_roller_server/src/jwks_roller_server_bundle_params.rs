use std::sync::Arc;

use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

pub struct JwksRollerServerBundleParams {
    pub storage: Arc<dyn JwksSecretStorage>,
}
