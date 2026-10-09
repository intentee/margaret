use std::future::ready;
use std::sync::Arc;

use futures_util::TryFutureExt as _;

use margaret_database::database::Database;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_creation::SigningKeysCreation;
use margaret_signing_keys::signing_keys_replacement::SigningKeysReplacement;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys::stored_signing_keys::StoredSigningKeys;

use crate::held_secret::HeldSecret;
use crate::restored_secret::restored_secret;
use crate::roll_due_at::roll_due_at;
use crate::roller_error::RollerError;
use crate::signing_key_retention::signing_key_retention;

fn confirm_progression(held: &JwksSecret, stored: &JwksSecret) -> Result<(), RollerError> {
    if stored.generation() < held.generation() {
        Err(RollerError::GenerationRegressed {
            held: held.generation(),
            stored: stored.generation(),
        })
    } else if stored.generation() == held.generation()
        && (stored.current().kid() != held.current().kid()
            || stored.next().kid() != held.next().kid())
    {
        Err(RollerError::GenerationForked {
            generation: held.generation(),
        })
    } else {
        Ok(())
    }
}

pub struct SigningKeysSynchronizer {
    pub curve: SigningCurve,
    pub database: Arc<Database>,
    pub rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
}

impl SigningKeysSynchronizer {
    /// # Errors
    ///
    /// Returns `RollerError` when the stored keys cannot be loaded, restored, created or replaced,
    /// when they do not progress from the held secret, or when a conflicting write the store
    /// reports cannot be observed.
    pub async fn synchronized(
        &self,
        held: &HeldSecret,
        now: NumericDate,
    ) -> Result<Arc<JwksSecret>, RollerError> {
        match self.loaded().await? {
            StoredSigningKeys::Absent => match held {
                HeldSecret::Held(secret) => Err(RollerError::StoredKeysVanished {
                    generation: secret.generation(),
                }),
                HeldSecret::Unheld => self.created(now).await,
            },
            StoredSigningKeys::Stored(revision) => {
                let stored = restored_secret(&revision, self.curve)?;

                if let HeldSecret::Held(secret) = held {
                    confirm_progression(secret, &stored)?;
                }

                if now < roll_due_at(&stored) {
                    Ok(Arc::new(stored))
                } else {
                    self.replaced(&stored, now).await
                }
            }
        }
    }

    async fn created(&self, now: NumericDate) -> Result<Arc<JwksSecret>, RollerError> {
        let fresh = JwksSecret::fresh(
            self.curve,
            self.rsa_keys.as_ref(),
            signing_key_retention(),
            now,
        )
        .map_err(RollerError::KeyGeneration)?;

        match ready(
            SigningKeysRevision::from_secret(&fresh).map_err(RollerError::DocumentSerialization),
        )
        .and_then(|revision| async move {
            SigningKeySet::create(&self.database, &revision)
                .await
                .map_err(|source| RollerError::SecretCreate { source })
        })
        .await?
        {
            SigningKeysCreation::Created => Ok(Arc::new(fresh)),
            SigningKeysCreation::AlreadyCreated => match self.loaded().await? {
                StoredSigningKeys::Absent => Err(RollerError::CreatedKeysNotObserved),
                StoredSigningKeys::Stored(revision) => {
                    restored_secret(&revision, self.curve).map(Arc::new)
                }
            },
        }
    }

    async fn loaded(&self) -> Result<StoredSigningKeys, RollerError> {
        SigningKeySet::load(&self.database)
            .await
            .map_err(|source| RollerError::SecretLoad { source })
    }

    async fn replaced(
        &self,
        stored: &JwksSecret,
        now: NumericDate,
    ) -> Result<Arc<JwksSecret>, RollerError> {
        let rolled = stored
            .rolled(self.rsa_keys.as_ref(), now)
            .map_err(RollerError::KeyRoll)?;

        match ready(
            SigningKeysRevision::from_secret(&rolled).map_err(RollerError::DocumentSerialization),
        )
        .and_then(|revision| async move {
            SigningKeySet::replace(&self.database, stored.generation(), &revision)
                .await
                .map_err(|source| RollerError::SecretReplace { source })
        })
        .await?
        {
            SigningKeysReplacement::Replaced => Ok(Arc::new(rolled)),
            SigningKeysReplacement::Superseded => match self.loaded().await? {
                StoredSigningKeys::Absent => Err(RollerError::StoredKeysVanished {
                    generation: stored.generation(),
                }),
                StoredSigningKeys::Stored(revision)
                    if revision.generation > stored.generation() =>
                {
                    restored_secret(&revision, self.curve).map(Arc::new)
                }
                StoredSigningKeys::Stored(revision) => {
                    Err(RollerError::SupersedingKeysNotObserved {
                        expected: stored.generation(),
                        observed: revision.generation,
                    })
                }
            },
        }
    }
}
