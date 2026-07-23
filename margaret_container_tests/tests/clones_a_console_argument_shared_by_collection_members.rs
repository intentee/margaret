use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn clones_a_console_argument_shared_by_collection_members() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/console_argument_collection_fanout");
    let source: String = generate_container_source("crate", &directory)
        .expect("the collection fan-out crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains(
        "vec![self.alpha_plugin(console_argument_0.clone()).await,self.beta_plugin(console_argument_0).await]"
    ));
}
