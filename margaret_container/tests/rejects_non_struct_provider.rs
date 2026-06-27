use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn rejects_non_struct_provider() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provider_not_a_struct");
    let error = generate_container_source("provider_not_a_struct", &directory)
        .err()
        .expect("a #[provider] on a non-struct must be rejected");

    assert!(matches!(error, ContainerError::ProviderNotAStruct { .. }));
}
