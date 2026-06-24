use std::path::Path;

use margaret_container::generate_container_source;

#[test]
fn constructs_dependencies_before_dependents() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("full", &directory)
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();
    let config_position = source
        .find("letconfig=")
        .expect("the config local is built");
    let greeter_position = source
        .find("letgreeter=")
        .expect("the greeter local is built");

    assert!(config_position < greeter_position);
}
