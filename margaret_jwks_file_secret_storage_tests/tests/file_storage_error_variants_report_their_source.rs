use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;

fn invalid_json_error() -> serde_json::Error {
    serde_json::from_str::<serde_json::Value>("not json")
        .expect_err("the fixture is not valid json")
}

#[test]
fn file_storage_error_variants_report_their_source() {
    let path = PathBuf::from("/secrets/jwks.json");

    let read = FileJwksSecretStorageError::Read {
        path: path.clone(),
        source: io::Error::new(ErrorKind::PermissionDenied, "denied"),
    };
    assert!(read.to_string().contains("/secrets/jwks.json"));
    assert!(read.source().is_some());

    let write = FileJwksSecretStorageError::Write {
        path: path.clone(),
        source: io::Error::new(ErrorKind::PermissionDenied, "denied"),
    };
    assert!(write.to_string().contains("/secrets/jwks.json"));
    assert!(write.source().is_some());

    let no_parent = FileJwksSecretStorageError::PathHasNoParentDirectory {
        path: PathBuf::from("/"),
    };
    assert!(no_parent.to_string().contains("filesystem root"));
    assert!(no_parent.source().is_none());

    let deserialize = FileJwksSecretStorageError::Deserialize {
        path: path.clone(),
        source: invalid_json_error(),
    };
    assert!(deserialize.to_string().contains("/secrets/jwks.json"));
    assert!(deserialize.source().is_some());

    let serialize = FileJwksSecretStorageError::Serialize(invalid_json_error());
    assert!(serialize.to_string().contains("serialize"));
    assert!(serialize.source().is_some());
}
