use std::path::Path;

use margaret_container::generate_container_source;

#[test]
fn wires_dependencies_through_lazy_accessors() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("full", &directory, &[])
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("crate::EnglishGreeter::new(self.config())"));
    assert!(source.contains("crate::App::new(self.greeter(),"));
}
