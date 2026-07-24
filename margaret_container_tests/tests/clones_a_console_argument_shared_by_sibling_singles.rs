use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn clones_a_console_argument_shared_by_sibling_singles() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_single_fanout");
    let source: String = generate_container_source("crate", &directory)
        .expect("the fan-out crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("self.first(console_argument_0.clone()).await"));
    assert!(source.contains("self.second(console_argument_0.clone()).await"));
    assert!(source.contains("self.third(console_argument_0).await"));
}
