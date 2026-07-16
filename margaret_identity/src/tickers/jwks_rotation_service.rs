use std::sync::Arc;

use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::roll::roll;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::scheduled_with_tick_timer;

use crate::identity_error::IdentityError;
use crate::signing_curve::SIGNING_CURVE;
use crate::stores::signing_key_store::SigningKeyStore;

#[scheduled_with_tick_timer(
    interval = crate::rotation_interval::ROTATION_INTERVAL,
    behavior = tokio::time::MissedTickBehavior::Delay
)]
pub struct JwksRotationService {
    store: Arc<SigningKeyStore>,
}

impl JwksRotationService {
    #[constructor]
    pub fn create(store: Arc<SigningKeyStore>) -> Self {
        Self { store }
    }

    #[process]
    pub async fn run(
        &self,
        #[console_argument(from = "jwks-secret-path")] jwks_secret_path: std::path::PathBuf,
    ) -> Result<(), IdentityError> {
        let storage = FileJwksSecretStorage::new(jwks_secret_path);

        roll(&storage, self.store.holder(), SIGNING_CURVE)?;

        Ok(())
    }
}
