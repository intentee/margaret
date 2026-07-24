use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_roller::roller_error::RollerError;

use crate::file_jwks_secret_storage_error::FileJwksSecretStorageError;

fn secret_persist_error(source: FileJwksSecretStorageError) -> RollerError {
    RollerError::SecretPersist {
        source: Box::new(source),
    }
}

pub struct FileJwksSecretStorage {
    path: PathBuf,
}

impl FileJwksSecretStorage {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<LoadedSecret, RollerError> {
        let persisted: PersistedJwksSecret =
            serde_json::from_slice(bytes).map_err(|source| RollerError::SecretLoad {
                source: Box::new(FileJwksSecretStorageError::Deserialize {
                    path: self.path.clone(),
                    source,
                }),
            })?;

        Ok(LoadedSecret::Present(Box::new(persisted.into_secret())))
    }

    fn persist_document(&self, secret: &JwksSecret) -> Result<(), FileJwksSecretStorageError> {
        serde_json::to_vec(&PersistedJwksSecret::new(secret.clone()))
            .map_err(FileJwksSecretStorageError::Serialize)
            .and_then(|document| self.write_document(&document))
    }

    fn write_document(&self, document: &[u8]) -> Result<(), FileJwksSecretStorageError> {
        self.write_securely(document)
            .map_err(|source| FileJwksSecretStorageError::Write {
                path: self.path.clone(),
                source,
            })
    }

    fn write_securely(&self, document: &[u8]) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&self.path)?;

        file.write_all(document)
    }
}

impl JwksSecretStorage for FileJwksSecretStorage {
    fn load(&self) -> Result<LoadedSecret, RollerError> {
        match std::fs::read(&self.path) {
            Ok(bytes) => self.deserialize(&bytes),
            Err(source) if source.kind() == ErrorKind::NotFound => Ok(LoadedSecret::Absent),
            Err(source) => Err(RollerError::SecretLoad {
                source: Box::new(FileJwksSecretStorageError::Read {
                    path: self.path.clone(),
                    source,
                }),
            }),
        }
    }

    fn persist(&self, secret: &JwksSecret) -> Result<(), RollerError> {
        self.persist_document(secret).map_err(secret_persist_error)
    }
}
