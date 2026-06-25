use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn rejects_singleton_on_method() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/singleton_on_method");
    let error = generate_container_source("singleton_on_method", &directory)
        .err()
        .expect("a #[singleton] on a method must be rejected");

    assert!(matches!(error, ContainerError::NotASingletonStruct { .. }));
}
