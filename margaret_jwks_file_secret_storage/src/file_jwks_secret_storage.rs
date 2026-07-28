use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_roller::roller_error::RollerError;
use tempfile::NamedTempFile;
use zeroize::Zeroizing;

use crate::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use crate::temporary_file_directory::temporary_file_directory;

fn write_atomically(directory: &Path, path: &Path, document: &[u8]) -> std::io::Result<()> {
    let mut file = NamedTempFile::new_in(directory)?;

    file.write_all(document)
        .and_then(|()| file.as_file().sync_all())
        .and_then(|()| file.persist(path).map_err(std::io::Error::from))
        .map(|_| ())
}

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
            .map(Zeroizing::new)
            .map_err(FileJwksSecretStorageError::Serialize)
            .and_then(|document| self.write_securely(&document))
    }

    fn write_securely(&self, document: &[u8]) -> Result<(), FileJwksSecretStorageError> {
        let directory = temporary_file_directory(&self.path)?;

        write_atomically(directory, &self.path, document).map_err(|source| {
            FileJwksSecretStorageError::Write {
                path: self.path.clone(),
                source,
            }
        })
    }
}

impl JwksSecretStorage for FileJwksSecretStorage {
    fn load(&self) -> anyhow::Result<LoadedSecret> {
        match std::fs::read(&self.path) {
            Ok(bytes) => self.deserialize(&bytes).map_err(Into::into),
            Err(source) if source.kind() == ErrorKind::NotFound => Ok(LoadedSecret::Absent),
            Err(source) => Err(FileJwksSecretStorageError::Read {
                path: self.path.clone(),
                source,
            }
            .into()),
        }
    }

    fn persist(&self, secret: &JwksSecret) -> anyhow::Result<()> {
        self.persist_document(secret)
            .map_err(secret_persist_error)
            .map_err(Into::into)
    }
}
