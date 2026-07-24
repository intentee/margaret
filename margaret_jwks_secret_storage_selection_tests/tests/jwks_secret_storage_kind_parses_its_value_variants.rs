use clap::ValueEnum;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_kind::JwksSecretStorageKind;

#[test]
fn jwks_secret_storage_kind_parses_its_value_variants() {
    assert_eq!(
        JwksSecretStorageKind::value_variants(),
        &[JwksSecretStorageKind::File, JwksSecretStorageKind::Memory]
    );
    assert_eq!(
        JwksSecretStorageKind::from_str("memory", false),
        Ok(JwksSecretStorageKind::Memory)
    );
    assert_eq!(
        JwksSecretStorageKind::from_str("file", false),
        Ok(JwksSecretStorageKind::File)
    );
    assert!(JwksSecretStorageKind::from_str("vault", false).is_err());
}
