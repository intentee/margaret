use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_argument_codegen::scan::scan;
use margaret_container::container_error::ContainerError;
use margaret_container::render_container::render_container;

#[test]
fn reports_a_framework_provider_colliding_with_a_user_singleton() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/framework_provider_collision");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");
    let framework_provided = vec![CanonicalPath::new(vec![
        "crate".to_string(),
        "Widget".to_string(),
    ])];

    let error = render_container(&index, &registry, &framework_provided)
        .err()
        .expect("a framework provider colliding with a user singleton must be rejected");

    assert!(matches!(
        error,
        ContainerError::AmbiguousFrameworkProvider { .. }
    ));
}
