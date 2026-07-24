use std::path::PathBuf;

use clap::Arg;
use clap::Command;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;

#[test]
fn jwks_secret_storage_uri_is_a_clap_value_parser() {
    let command = Command::new("app").arg(
        Arg::new("storage")
            .long("storage")
            .value_parser(clap::value_parser!(JwksSecretStorageUri)),
    );

    let matches = command
        .clone()
        .try_get_matches_from(["app", "--storage", "file:/tmp/jwks.json"])
        .expect("a valid storage uri parses");

    assert_eq!(
        matches.get_one::<JwksSecretStorageUri>("storage"),
        Some(&JwksSecretStorageUri::File {
            path: PathBuf::from("/tmp/jwks.json"),
        })
    );

    assert!(
        command
            .try_get_matches_from(["app", "--storage", "vault:/x"])
            .is_err()
    );
}
