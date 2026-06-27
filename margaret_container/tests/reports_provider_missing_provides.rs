use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_provider_missing_provides() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provider_missing_provides");
    let error = generate_container_source("provider_missing_provides", &directory)
        .err()
        .expect("a #[provider] without 'provides' must be rejected");

    assert!(matches!(
        error,
        ContainerError::ProviderMissingProvides { .. }
    ));
}
