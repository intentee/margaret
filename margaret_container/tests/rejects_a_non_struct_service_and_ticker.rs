use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn rejects_a_non_struct_service() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/service_not_a_struct");
    let error = generate_container_source("crate", &directory)
        .expect_err("a #[service] on a non-struct must be rejected");

    assert!(matches!(error, ContainerError::NotASingletonStruct { .. }));
}

#[test]
fn rejects_a_non_struct_ticker() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ticker_not_a_struct");
    let error = generate_container_source("crate", &directory)
        .expect_err("a #[scheduled_with_tick_timer] on a non-struct must be rejected");

    assert!(matches!(error, ContainerError::NotASingletonStruct { .. }));
}
