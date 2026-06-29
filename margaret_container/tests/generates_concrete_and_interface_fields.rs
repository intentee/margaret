use std::path::Path;

use margaret_container::generate_container_source::generate_container_source;

#[test]
fn generates_concrete_and_interface_fields() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("crate", &directory)
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("config:tokio::sync::OnceCell<std::sync::Arc<crate::Config>>"));
    assert!(source.contains("greeter:tokio::sync::OnceCell<std::sync::Arc<dyncrate::Greeter>>"));
}
