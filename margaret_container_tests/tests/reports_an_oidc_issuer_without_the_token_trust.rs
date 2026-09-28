use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_an_oidc_issuer_without_the_token_trust() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/oidc_issuer_missing_trait");

    assert!(matches!(
        generate_container_source("oidc_issuer_missing_trait", &directory),
        Err(ContainerError::DeclarationMissingTrait {
            attribute: "trusts_oidc_issuer",
            ..
        })
    ));
}
