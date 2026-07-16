use std::io::ErrorKind;
use std::io::Result as IoResult;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::path::PathBuf;

use serde::Serialize;

use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::persisted_jwks_secret::PersistedJwksSecret;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::loaded_secret::LoadedSecret;
use crate::roller_error::RollerError;

fn write_new_owner_only(path: &Path, bytes: &[u8]) -> IoResult<()> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;

    file.write_all(bytes)
}

pub struct FileJwksSecretStorage {
    path: PathBuf,
}

impl FileJwksSecretStorage {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn persist_value<Value: Serialize>(&self, value: &Value) -> Result<(), RollerError> {
        let bytes =
            serde_json::to_vec(value).map_err(|source| RollerError::Serialize { source })?;

        self.write_atomically(&bytes)
    }

    fn temporary_path(&self) -> PathBuf {
        let mut temporary = self.path.clone().into_os_string();
        temporary.push(".tmp");

        PathBuf::from(temporary)
    }

    fn write_atomically(&self, bytes: &[u8]) -> Result<(), RollerError> {
        let temporary_path = self.temporary_path();

        write_new_owner_only(&temporary_path, bytes)
            .and_then(|()| std::fs::rename(&temporary_path, &self.path))
            .map_err(|source| RollerError::Write {
                path: self.path.display().to_string(),
                source,
            })
    }
}

impl JwksSecretStorage for FileJwksSecretStorage {
    fn load(&self) -> Result<LoadedSecret, RollerError> {
        let bytes = match std::fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(LoadedSecret::Absent),
            Err(source) => {
                return Err(RollerError::Read {
                    path: self.path.display().to_string(),
                    source,
                });
            }
        };

        serde_json::from_slice::<PersistedJwksSecret>(&bytes)
            .map(|persisted| LoadedSecret::Present(Box::new(persisted.into_secret())))
            .map_err(|source| RollerError::Deserialize {
                path: self.path.display().to_string(),
                source,
            })
    }

    fn persist(&self, secret: &JwksSecret) -> Result<(), RollerError> {
        self.persist_value(&PersistedJwksSecret::new(secret.clone()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde::Serialize;
    use serde::Serializer;
    use serde::ser::Error;

    use super::FileJwksSecretStorage;

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target>(&self, _serializer: Target) -> Result<Target::Ok, Target::Error>
        where
            Target: Serializer,
        {
            Err(Target::Error::custom("this value cannot be serialized"))
        }
    }

    #[test]
    fn persist_value_reports_a_serialize_error_for_an_unserializable_value() {
        let storage = FileJwksSecretStorage::new(PathBuf::from("unused.json"));

        let error = storage
            .persist_value(&Unserializable)
            .expect_err("serializing an unserializable value fails");

        assert_eq!(
            error.to_string(),
            "failed to serialize the jwks secret: this value cannot be serialized"
        );
    }
}
