use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::insertion::Insertion;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::secret_text::SecretText;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

use crate::signing_key_set_name::SIGNING_KEY_SET_NAME;
use crate::signing_keys_creation::SigningKeysCreation;
use crate::signing_keys_error::SigningKeysError;
use crate::signing_keys_replacement::SigningKeysReplacement;
use crate::signing_keys_revision::SigningKeysRevision;
use crate::stored_signing_keys::StoredSigningKeys;

fn stored_generation(generation: SigningKeysGeneration) -> Result<i64, SigningKeysError> {
    i64::try_from(generation.value())
        .map_err(|source| SigningKeysError::GenerationOutOfRange { generation, source })
}

fn stored_signing_keys(
    lookup: Lookup<SigningKeySet>,
) -> Result<StoredSigningKeys, SigningKeysError> {
    match lookup {
        Lookup::Found(SigningKeySet {
            document,
            generation,
            ..
        }) => u64::try_from(generation)
            .map(|value| {
                StoredSigningKeys::Stored(SigningKeysRevision {
                    document,
                    generation: SigningKeysGeneration::new(value),
                })
            })
            .map_err(|source| SigningKeysError::StoredGenerationOutOfRange { generation, source }),
        Lookup::Missing => Ok(StoredSigningKeys::Absent),
    }
}

fn creation(insertion: Insertion) -> SigningKeysCreation {
    match insertion {
        Insertion::AlreadyPresent => SigningKeysCreation::AlreadyCreated,
        Insertion::Inserted => SigningKeysCreation::Created,
    }
}

fn replacement(change: &Change<SigningKeySet>) -> SigningKeysReplacement {
    match change {
        Change::Changed(_) => SigningKeysReplacement::Replaced,
        Change::Unmatched => SigningKeysReplacement::Superseded,
    }
}

#[model(table = "signing_key_sets")]
pub struct SigningKeySet {
    #[column(primary_key)]
    pub name: String,
    #[column]
    pub generation: i64,
    #[column]
    pub document: SecretText,
}

impl SigningKeySet {
    /// # Errors
    ///
    /// Returns `SigningKeysError::Load` when the signing keys cannot be read, and
    /// `SigningKeysError::StoredGenerationOutOfRange` when their generation is no roll's.
    pub async fn load(database: &Database) -> Result<StoredSigningKeys, SigningKeysError> {
        Self::query()
            .name
            .eq(SIGNING_KEY_SET_NAME.to_string())
            .find(database)
            .await
            .map_err(SigningKeysError::Load)
            .and_then(stored_signing_keys)
    }

    /// # Errors
    ///
    /// Returns `SigningKeysError::GenerationOutOfRange` when the generation does not fit the
    /// database, and `SigningKeysError::Create` when the signing keys cannot be stored.
    pub async fn create(
        database: &Database,
        SigningKeysRevision {
            document,
            generation,
        }: &SigningKeysRevision,
    ) -> Result<SigningKeysCreation, SigningKeysError> {
        let generation = stored_generation(*generation)?;

        Self {
            name: SIGNING_KEY_SET_NAME.to_string(),
            generation,
            document: document.clone(),
        }
        .insert()
        .or_ignore(database)
        .await
        .map(creation)
        .map_err(SigningKeysError::Create)
    }

    /// # Errors
    ///
    /// Returns `SigningKeysError::GenerationOutOfRange` when a generation does not fit the
    /// database, and `SigningKeysError::Replace` when the signing keys cannot be replaced.
    pub async fn replace(
        database: &Database,
        expected: SigningKeysGeneration,
        SigningKeysRevision {
            document,
            generation,
        }: &SigningKeysRevision,
    ) -> Result<SigningKeysReplacement, SigningKeysError> {
        let expected = stored_generation(expected)?;
        let generation = stored_generation(*generation)?;

        Self::query()
            .name
            .eq(SIGNING_KEY_SET_NAME.to_string())
            .when(|keys| keys.generation.eq(expected))
            .update(database, |columns| {
                columns
                    .generation
                    .to(generation)
                    .and(columns.document.to(document.clone()))
            })
            .await
            .map(|change| replacement(&change))
            .map_err(SigningKeysError::Replace)
    }
}
