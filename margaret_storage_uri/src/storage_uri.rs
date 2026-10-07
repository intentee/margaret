use std::path::PathBuf;
use std::str::FromStr;

use url::Url;

use crate::storage_uri_error::StorageUriError;

const FILE_SCHEME: &str = "file";
const MEMORY_SCHEME: &str = "memory";
const POSTGRES_SCHEME: &str = "postgres";
const POSTGRESQL_SCHEME: &str = "postgresql";

fn is_bare(url: &Url) -> bool {
    url.host().is_none()
        && url.path().is_empty()
        && url.query().is_none()
        && url.fragment().is_none()
}

pub enum StorageUri {
    File { path: PathBuf },
    Memory,
    Postgres { url: Url },
}

impl FromStr for StorageUri {
    type Err = StorageUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value).map_err(|source| StorageUriError::Malformed { source })?;

        match url.scheme() {
            FILE_SCHEME => match (url.query(), url.fragment(), url.to_file_path()) {
                (None, None, Ok(path)) => Ok(Self::File { path }),
                (_, _, _) => Err(StorageUriError::FileNotAbsolutePath),
            },
            MEMORY_SCHEME if is_bare(&url) => Ok(Self::Memory),
            MEMORY_SCHEME => Err(StorageUriError::MemoryWithLocation),
            POSTGRES_SCHEME | POSTGRESQL_SCHEME => Ok(Self::Postgres { url }),
            scheme => Err(StorageUriError::UnknownScheme {
                scheme: scheme.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;
    use std::path::Path;

    use super::StorageUri;
    use crate::storage_uri_error::StorageUriError;

    fn parsed(value: &str) -> Result<StorageUri, StorageUriError> {
        value.parse()
    }

    fn rejection(value: &str) -> StorageUriError {
        parsed(value).err().expect("the storage uri is rejected")
    }

    #[test]
    fn reads_the_memory_storage() {
        assert_eq!(
            discriminant(&parsed("memory:").expect("memory is a storage")),
            discriminant(&StorageUri::Memory)
        );
    }

    #[test]
    fn rejects_a_location_of_the_memory_storage() {
        assert_eq!(
            discriminant(&rejection("memory:jwks")),
            discriminant(&StorageUriError::MemoryWithLocation)
        );
    }

    #[test]
    fn reads_an_absolute_file_path() {
        assert!(matches!(
            parsed("file:///var/lib/app/jwks.json"),
            Ok(StorageUri::File { path }) if path == Path::new("/var/lib/app/jwks.json")
        ));
    }

    #[test]
    fn rejects_a_file_on_another_host() {
        assert_eq!(
            discriminant(&rejection("file://storage.example/jwks.json")),
            discriminant(&StorageUriError::FileNotAbsolutePath)
        );
    }

    #[test]
    fn rejects_a_file_path_with_a_query() {
        assert_eq!(
            discriminant(&rejection("file:///var/lib/app/jwks.json?mode=0600")),
            discriminant(&StorageUriError::FileNotAbsolutePath)
        );
    }

    #[test]
    fn reads_both_postgres_schemes() {
        assert!(matches!(
            parsed("postgres://app@db.example/state"),
            Ok(StorageUri::Postgres { url }) if url.host_str() == Some("db.example")
        ));
        assert!(matches!(
            parsed("postgresql://app@db.example/state"),
            Ok(StorageUri::Postgres { url }) if url.scheme() == "postgresql"
        ));
    }

    #[test]
    fn rejects_an_unknown_scheme() {
        assert!(matches!(
            rejection("redis://cache.example"),
            StorageUriError::UnknownScheme { scheme } if scheme == "redis"
        ));
    }

    #[test]
    fn keeps_the_value_out_of_a_malformed_uri_error() {
        let error = rejection("hunter2");

        assert!(!error.to_string().contains("hunter2"));
        assert!(matches!(
            error,
            StorageUriError::Malformed { source } if source == url::ParseError::RelativeUrlWithoutBase
        ));
    }
}
