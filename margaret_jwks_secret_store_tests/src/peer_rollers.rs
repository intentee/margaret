use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roll_due_at::roll_due_at;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;

pub struct PeerRollers {
    pub lagging: Arc<JwksRoller>,
    pub rolled: Arc<JwksRoller>,
}

impl PeerRollers {
    /// # Panics
    ///
    /// Panics when the peers cannot start or the keys cannot roll between their starts.
    pub async fn one_generation_apart() -> Self {
        let storage = Arc::new(FixtureSigningKeys::empty());
        let lagging =
            JwksRoller::create(storage.clone(), Arc::new(FixtureRsaSigningKeys::default()))
                .await
                .expect("the lagging peer starts");
        let held = lagging.jwks_secret_holder().get();

        fixture_synchronizer(storage.clone())
            .synchronized(&HeldSecret::Held(held.clone()), roll_due_at(&held))
            .await
            .expect("another peer rolls the keys");

        Self {
            lagging: Arc::new(lagging),
            rolled: Arc::new(
                JwksRoller::create(storage, Arc::new(FixtureRsaSigningKeys::default()))
                    .await
                    .expect("the rolled peer starts"),
            ),
        }
    }
}
