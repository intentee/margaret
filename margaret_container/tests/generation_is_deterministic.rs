use std::path::Path;

use margaret_container::generate_container_source;

#[test]
fn generation_is_deterministic() {
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
