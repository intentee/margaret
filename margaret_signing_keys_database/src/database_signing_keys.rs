use std::future::ready;
use std::sync::Arc;

use async_trait::async_trait;
use futures_util::TryFutureExt as _;
use tokio_postgres::Row;
use zeroize::Zeroizing;

use margaret_database::database::Database;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_document::SigningKeysDocument;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_signing_keys_schema::signing_key_set::SigningKeySet;

use crate::signing_key_set_name::SIGNING_KEY_SET_NAME;
use crate::signing_keys_database_error::SigningKeysDatabaseError;
use crate::signing_keys_statements::SigningKeysStatements;

fn stored_generation(generation: SigningKeysGeneration) -> Result<i64, SigningKeysDatabaseError> {
    i64::try_from(generation.value())
        .map_err(|source| SigningKeysDatabaseError::GenerationOutOfRange { generation, source })
}

fn signing_key_set(row: &Row) -> Result<SigningKeySet, SigningKeysDatabaseError> {
    row.try_get("name")
        .and_then(|name| {
            row.try_get("generation").and_then(|generation| {
                row.try_get("document").map(|document| SigningKeySet {
                    document,
                    generation,
                    name,
                })
            })
        })
        .map_err(SigningKeysDatabaseError::MalformedRow)
}

fn stored_revision(
    SigningKeySet {
        document,
        generation,
        ..
    }: SigningKeySet,
) -> Result<SigningKeysRevision, SigningKeysDatabaseError> {
    u64::try_from(generation)
        .map(|value| SigningKeysRevision {
            document: SigningKeysDocument::new(Zeroizing::new(document)),
            generation: SigningKeysGeneration::new(value),
        })
        .map_err(
            |source| SigningKeysDatabaseError::StoredGenerationOutOfRange { generation, source },
        )
}

pub struct DatabaseSigningKeys {
    database: Arc<Database>,
    statements: SigningKeysStatements,
}

impl DatabaseSigningKeys {
    #[must_use]
    pub fn create(database: Arc<Database>) -> Self {
        Self {
            database,
            statements: SigningKeysStatements::new(),
        }
    }
}

#[async_trait]
impl StoresSigningKeys for DatabaseSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(self
            .database
            .client()
            .map_err(SigningKeysDatabaseError::Unavailable)
            .and_then(|client| async move {
                client
                    .query_opt(&self.statements.load, &[&SIGNING_KEY_SET_NAME])
                    .await
                    .map_err(SigningKeysDatabaseError::Load)
            })
            .await
            .and_then(|row| match row {
                None => Ok(StoredSigningKeys::Absent),
                Some(row) => signing_key_set(&row)
                    .and_then(stored_revision)
                    .map(StoredSigningKeys::Stored),
            })?)
    }

    async fn create_signing_keys(
        &self,
        SigningKeysRevision {
            document,
            generation,
        }: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation> {
        let created = ready(stored_generation(*generation))
            .and_then(|generation| async move {
                self.database
                    .client()
                    .map_err(SigningKeysDatabaseError::Unavailable)
                    .and_then(|client| async move {
                        client
                            .execute(
                                &self.statements.create,
                                &[&SIGNING_KEY_SET_NAME, &generation, &document.json()],
                            )
                            .await
                            .map_err(SigningKeysDatabaseError::Create)
                    })
                    .await
            })
            .await?;

        Ok(match created {
            0 => SigningKeysCreation::AlreadyCreated,
            _ => SigningKeysCreation::Created,
        })
    }

    async fn replace_signing_keys(
        &self,
        expected: SigningKeysGeneration,
        SigningKeysRevision {
            document,
            generation,
        }: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysReplacement> {
        let replaced = ready(stored_generation(expected).and_then(|expected| {
            stored_generation(*generation).map(|generation| [expected, generation])
        }))
        .and_then(|[expected, generation]| async move {
            self.database
                .client()
                .map_err(SigningKeysDatabaseError::Unavailable)
                .and_then(|client| async move {
                    client
                        .execute(
                            &self.statements.replace,
                            &[
                                &SIGNING_KEY_SET_NAME,
                                &expected,
                                &generation,
                                &document.json(),
                            ],
                        )
                        .await
                        .map_err(SigningKeysDatabaseError::Replace)
                })
                .await
        })
        .await?;

        Ok(match replaced {
            0 => SigningKeysReplacement::Superseded,
            _ => SigningKeysReplacement::Replaced,
        })
    }
}
