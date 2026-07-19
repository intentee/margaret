use std::sync::Arc;

use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::scheduled_with_tick_timer;

use crate::jwks_rolling::JwksRolling;

#[scheduled_with_tick_timer(
    interval = margaret_jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL,
    behavior = tokio::time::MissedTickBehavior::Delay
)]
pub struct JwksRoller {
    jwks_rolling: Arc<JwksRolling>,
}

impl JwksRoller {
    #[constructor]
    #[must_use]
    pub fn create(jwks_rolling: Arc<JwksRolling>) -> Self {
        Self { jwks_rolling }
    }

    #[process]
    pub async fn run(&self) -> Result<(), JwksRollerServerError> {
        self.jwks_rolling.roll_and_publish()
    }
}
