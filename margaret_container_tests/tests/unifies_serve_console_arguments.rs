use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn path(name: &str) -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
}

#[test]
fn unifies_serve_console_arguments_across_roots() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let bindings = bindings_for_fixture("crate", &directory);

    let serve = bindings
        .serve_arguments(&[path("Config"), path("Service")], &[])
        .expect("the roots unify");
    let names: Vec<&str> = serve.iter().map(|argument| argument.name()).collect();

    assert_eq!(serve.len(), 2);
    assert!(names.contains(&"path"));
    assert!(names.contains(&"alpha"));

    assert_eq!(bindings.console_arguments(&path("Config")).len(), 1);
    assert!(bindings.console_arguments(&path("Absent")).is_empty());

    let path_slot = bindings.console_slot("path");
    let alpha_slot = bindings.console_slot("alpha");

    assert_ne!(path_slot, alpha_slot);
}
