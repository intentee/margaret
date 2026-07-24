use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn rejects_singleton_arguments() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/singleton_with_arguments");
    let error = generate_container_source("crate", &directory)
        .expect_err("#[singleton] with arguments must be rejected");

    assert!(matches!(error, ContainerError::SingletonHasArguments { .. }));
}
