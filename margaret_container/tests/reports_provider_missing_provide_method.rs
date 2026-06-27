use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_provider_missing_provide_method() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provider_missing_provide");
    let error = generate_container_source("provider_missing_provide", &directory)
        .err()
        .expect("a #[provider] without a #[provide] method must be rejected");

    assert!(matches!(
        error,
        ContainerError::ProviderMissingProvideMethod { .. }
    ));
}
