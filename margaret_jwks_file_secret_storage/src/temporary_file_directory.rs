use std::path::Path;

use crate::file_jwks_secret_storage_error::FileJwksSecretStorageError;

pub(crate) fn temporary_file_directory(path: &Path) -> Result<&Path, FileJwksSecretStorageError> {
    match path.parent() {
        Some(parent) if parent.as_os_str().is_empty() => Ok(Path::new(".")),
        Some(parent) => Ok(parent),
        None => Err(FileJwksSecretStorageError::PathHasNoParentDirectory {
            path: path.to_path_buf(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::temporary_file_directory;

    fn resolved(path: &str) -> Result<String, String> {
        match temporary_file_directory(Path::new(path)) {
            Ok(directory) => Ok(directory.display().to_string()),
            Err(error) => Err(error.to_string()),
        }
    }

    #[test]
    fn resolves_the_directory_that_holds_the_target_file() {
        assert_eq!(resolved("/secrets/jwks.json"), Ok("/secrets".to_string()));
    }

    #[test]
    fn resolves_a_bare_relative_target_to_the_current_directory() {
        assert_eq!(resolved("jwks.json"), Ok(".".to_string()));
    }

    #[test]
    fn rejects_a_target_that_is_a_filesystem_root() {
        assert_eq!(
            resolved("/"),
            Err(
                "the jwks secret path '/' is a filesystem root and has no parent directory"
                    .to_string()
            )
        );
    }
}
