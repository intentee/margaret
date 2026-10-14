use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::insertion::Insertion;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys_encryption::sealed_document::SealedDocument;

use crate::constants::SIGNING_KEY_SET_NAME;
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
    let Lookup::Found(SigningKeySet {
        generation,
        nonce,
        sealed_document,
        ..
    }) = lookup
    else {
        return Ok(StoredSigningKeys::Absent);
    };
    let generation = u64::try_from(generation)
        .map_err(|source| SigningKeysError::StoredGenerationOutOfRange { generation, source })?;
    let nonce = nonce
        .try_into()
        .map_err(|nonce: Vec<u8>| SigningKeysError::StoredNonceLength {
            length: nonce.len(),
        })?;

    Ok(StoredSigningKeys::Stored(SigningKeysRevision {
        generation: SigningKeysGeneration::new(generation),
        sealed: SealedDocument {
            ciphertext: sealed_document,
            nonce,
        },
    }))
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
    #[column(byte_length = 12)]
    pub nonce: Vec<u8>,
    #[column]
    pub sealed_document: Vec<u8>,
}

impl SigningKeySet {
    /// # Errors
    ///
    /// Returns `SigningKeysError::GenerationOutOfRange` when the generation does not fit the
    /// database, and `SigningKeysError::Create` when the signing keys cannot be stored.
    pub async fn create(
        database: &Database,
        SigningKeysRevision { generation, sealed }: &SigningKeysRevision,
    ) -> Result<SigningKeysCreation, SigningKeysError> {
        let generation = stored_generation(*generation)?;

        Self {
            name: SIGNING_KEY_SET_NAME.to_string(),
            generation,
            nonce: sealed.nonce.to_vec(),
            sealed_document: sealed.ciphertext.clone(),
        }
        .insert()
        .or_ignore(database)
        .await
        .map(creation)
        .map_err(SigningKeysError::Create)
    }

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
    /// Returns `SigningKeysError::GenerationOutOfRange` when a generation does not fit the
    /// database, and `SigningKeysError::Replace` when the signing keys cannot be replaced.
    pub async fn replace(
        database: &Database,
        expected: SigningKeysGeneration,
        SigningKeysRevision { generation, sealed }: &SigningKeysRevision,
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
                    .and(columns.nonce.to(sealed.nonce.to_vec()))
                    .and(columns.sealed_document.to(sealed.ciphertext.clone()))
            })
            .await
            .map(|change| replacement(&change))
            .map_err(SigningKeysError::Replace)
    }
}
