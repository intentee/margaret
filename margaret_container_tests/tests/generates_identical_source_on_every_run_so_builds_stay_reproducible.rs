use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn generates_identical_source_on_every_run_so_builds_stay_reproducible() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let first = generate_container_source("full", &directory)
        .expect("the full fixture generates a container")
        .source()
        .to_string();
    let second = generate_container_source("full", &directory)
        .expect("the full fixture generates a container")
        .source()
        .to_string();

    assert_eq!(first, second);
}
