use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_ambiguous_provide_method() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ambiguous_provide");
    let error = generate_container_source("ambiguous_provide", &directory)
        .err()
        .expect("a #[provider] with multiple #[provide] methods must be rejected");

    assert!(matches!(
        error,
        ContainerError::AmbiguousProvideMethod { .. }
    ));
}
