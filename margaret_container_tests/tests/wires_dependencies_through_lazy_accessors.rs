use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn wires_dependencies_through_lazy_accessors() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("crate", &directory)
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("crate::EnglishGreeter::new(self.config().await?)"));
    assert!(source.contains("crate::App::new(self.english_greeter().await?,"));
}
