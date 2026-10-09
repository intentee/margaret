use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;

use crate::fixture_roller::fixture_roller;

pub async fn rolled_roller(secret: JwksSecret) -> Arc<JwksRoller> {
    let roller = fixture_roller().await;

    roller.jwks_secret_holder().set(Arc::new(secret));

    roller
}
