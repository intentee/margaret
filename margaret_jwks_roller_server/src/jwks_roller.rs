use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use chrono::Utc;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use margaret_deadline::await_deadline::await_deadline;
use margaret_deadline::deadline_wake::DeadlineWake;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roll_due_at::roll_due_at;
use margaret_jwks_roller::signing_keys_synchronizer::SigningKeysSynchronizer;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::jwks_curve::JWKS_CURVE;
use crate::jwks_document_holder::JwksDocumentHolder;
use crate::jwks_roller_server_error::JwksRollerServerError;
use crate::public_jwks_handler::PublicJwksHandler;

fn published_document(secret: &JwksSecret) -> Result<Bytes, JwksRollerServerError> {
    serde_json::to_vec(secret.public_jwks())
        .map(Bytes::from)
        .map_err(JwksRollerServerError::DocumentSerialization)
}

fn roll_deadline(secret: &JwksSecret, now: NumericDate, instant: Instant) -> Instant {
    instant
        + Duration::from_secs(
            roll_due_at(secret)
                .seconds_since_epoch()
                .saturating_sub(now.seconds_since_epoch())
                .max(0)
                .unsigned_abs(),
        )
        .min(JWKS_ROLL_INTERVAL)
}

fn wall_clock() -> NumericDate {
    NumericDate::from(Utc::now())
}

pub struct JwksRoller {
    jwks_document_holder: JwksDocumentHolder,
    jwks_secret_holder: JwksSecretHolder,
    public_jwks_handler: Arc<PublicJwksHandler>,
    synchronizer: SigningKeysSynchronizer,
}

impl JwksRoller {
    /// # Errors
    ///
    /// Returns `JwksRollerServerError::SecretRoll` when the stored keys cannot be synchronized,
    /// and `JwksRollerServerError::DocumentSerialization` when their public document cannot be
    /// serialized.
    pub async fn create(
        storage: Arc<dyn StoresSigningKeys>,
        rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
    ) -> Result<Self, JwksRollerServerError> {
        let synchronizer = SigningKeysSynchronizer {
            curve: JWKS_CURVE,
            rsa_keys,
            storage,
        };
        let secret = synchronizer
            .synchronized(&HeldSecret::Unheld, wall_clock())
            .await
            .map_err(JwksRollerServerError::SecretRoll)?;

        published_document(&secret).map(|document| {
            let jwks_document_holder = JwksDocumentHolder::new(document);

            Self {
                public_jwks_handler: Arc::new(PublicJwksHandler::new(jwks_document_holder.clone())),
                jwks_document_holder,
                jwks_secret_holder: JwksSecretHolder::new(secret),
                synchronizer,
            }
        })
    }

    #[must_use]
    pub fn jwks_document_holder(&self) -> JwksDocumentHolder {
        self.jwks_document_holder.clone()
    }

    #[must_use]
    pub fn jwks_secret_holder(&self) -> JwksSecretHolder {
        self.jwks_secret_holder.clone()
    }

    #[must_use]
    pub fn public_jwks_handler(&self) -> Arc<PublicJwksHandler> {
        self.public_jwks_handler.clone()
    }

    /// # Errors
    ///
    /// Returns `JwksRollerServerError::SecretRoll` when the stored keys cannot be synchronized,
    /// and `JwksRollerServerError::DocumentSerialization` when their public document cannot be
    /// serialized.
    pub async fn run(
        &self,
        cancellation_token: CancellationToken,
    ) -> Result<(), JwksRollerServerError> {
        while await_deadline(
            roll_deadline(&self.jwks_secret_holder.get(), wall_clock(), Instant::now()),
            &cancellation_token,
        )
        .await
            == DeadlineWake::Reached
        {
            self.synchronizer
                .synchronized(
                    &HeldSecret::Held(self.jwks_secret_holder.get()),
                    wall_clock(),
                )
                .await
                .map_err(JwksRollerServerError::SecretRoll)
                .and_then(|synchronized| self.published(synchronized))?;
        }

        Ok(())
    }

    fn published(&self, secret: Arc<JwksSecret>) -> Result<(), JwksRollerServerError> {
        published_document(&secret).map(|document| {
            self.jwks_document_holder.set(document);
            self.jwks_secret_holder.set(secret);
        })
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::Instant;

    use margaret_jwks_keygen::signing_curve::SigningCurve;
    use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
    use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
    use margaret_jwks_roller::roll_due_at::roll_due_at;
    use margaret_registered_claims::numeric_date::NumericDate;

    use super::roll_deadline;

    #[test]
    fn waits_until_the_roll_is_due() {
        let secret = fresh_secret(SigningCurve::P256);
        let instant = Instant::now();

        assert_eq!(
            roll_deadline(&secret, secret.rolled_at(), instant),
            instant + JWKS_ROLL_INTERVAL
        );
    }

    #[test]
    fn wakes_at_once_for_an_overdue_roll() {
        let secret = fresh_secret(SigningCurve::P256);
        let instant = Instant::now();

        assert_eq!(
            roll_deadline(
                &secret,
                roll_due_at(&secret).after(JWKS_ROLL_INTERVAL),
                instant
            ),
            instant
        );
    }

    #[test]
    fn resynchronizes_within_one_interval_of_keys_rolled_ahead_of_its_clock() {
        let secret = fresh_secret(SigningCurve::P256);
        let instant = Instant::now();

        assert_eq!(
            roll_deadline(&secret, NumericDate::new(i64::MIN), instant),
            instant + JWKS_ROLL_INTERVAL
        );
    }
}
