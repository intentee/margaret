use std::path::PathBuf;
use std::str::FromStr;

use crate::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[derive(Clone, Debug, PartialEq)]
pub enum JwksSecretStorageUri {
    File { path: PathBuf },
    Memory,
}

impl FromStr for JwksSecretStorageUri {
    type Err = JwksSecretStorageUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (scheme, rest) = match value.split_once(':') {
            Some((scheme, rest)) => (scheme, Some(rest)),
            None => (value, None),
        };

        match (scheme, rest) {
            ("memory", None | Some("")) => Ok(Self::Memory),
            ("memory", Some(path)) => Err(JwksSecretStorageUriError::MemoryTakesNoPath {
                path: path.to_string(),
            }),
            ("file", Some(path)) if !path.is_empty() => Ok(Self::File {
                path: PathBuf::from(path),
            }),
            ("file", _) => Err(JwksSecretStorageUriError::FileRequiresPath),
            (scheme, _) => Err(JwksSecretStorageUriError::UnknownScheme {
                scheme: scheme.to_string(),
            }),
        }
    }
}
