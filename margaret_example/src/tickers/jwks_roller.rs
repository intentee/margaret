use std::sync::Arc;

use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::scheduled_with_tick_timer;
use margaret_macros::singleton;

#[singleton]
#[scheduled_with_tick_timer(
    interval = margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL,
    behavior = tokio::time::MissedTickBehavior::Delay
)]
pub struct JwksRoller {
    bundle: JwksRollerServerBundle,
    public_jwks_handler: Arc<PublicJwksHandler>,
}

impl JwksRoller {
    #[constructor]
    #[must_use]
    pub fn create(storage: Arc<dyn JwksSecretStorage>) -> Self {
        let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams { storage });
        let public_jwks_handler = bundle.public_jwks_handler();

        Self {
            bundle,
            public_jwks_handler,
        }
    }

    #[must_use]
    pub fn public_jwks_handler(&self) -> &PublicJwksHandler {
        &self.public_jwks_handler
    }

    #[process]
    pub async fn run(&self) -> Result<(), JwksRollerServerError> {
        self.bundle.roll_and_publish()
    }
}
