use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source;

#[test]
fn rejects_non_struct_singleton() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/not_a_struct");
    let error = generate_container_source("not_a_struct", &directory)
        .err()
        .expect("a #[singleton] on a non-struct item must be rejected");

    assert!(matches!(error, ContainerError::NotASingletonStruct { .. }));
}
