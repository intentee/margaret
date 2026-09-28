use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_serve_input_codegen::scan::scan;

fn uri_selected_provider() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::UriSelected {
            argument_name: "test-storage".to_string(),
            resolver: CanonicalPath::new(vec![
                "crate".to_string(),
                "resolve_test_storage".to_string(),
            ]),
            value_type: CanonicalPath::new(vec!["crate".to_string(), "TestStorageUri".to_string()]),
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "TestStorage".to_string()]),
    }
}

fn render_with_provider(fixture: &str) -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");

    container_module_source(
        render_container(&index, &registry, &[uri_selected_provider()])
            .expect("the fixture renders")
            .modules,
    )
    .split_whitespace()
    .collect()
}

#[test]
fn materializes_the_provider_as_a_trait_object_field() {
    assert!(
        render_with_provider("uri_selected_provider")
            .contains("test_storage:::std::sync::Arc<dyncrate::TestStorage>")
    );
}

#[test]
fn parameterizes_the_root_builder_with_the_uri_console_argument() {
    assert!(
        render_with_provider("uri_selected_provider").contains(
            "pubfnconstruct_test_storage(super::construct_test_storage_arguments::ConstructTestStorageArguments{argument0:serve_input_0,}:super::construct_test_storage_arguments::ConstructTestStorageArguments,)"
        )
    );
}

#[test]
fn injects_the_resolver_result_without_rewrapping_it_in_a_new_arc() {
    let source = render_with_provider("uri_selected_provider");

    assert!(source.contains(
        "std::sync::Arc<dyncrate::TestStorage>=crate::resolve_test_storage(serve_input_0,)"
    ));
    assert!(!source.contains("Arc::new(crate::resolve_test_storage"));
}

#[test]
fn skips_a_provider_that_no_singleton_references() {
    assert!(!render_with_provider("fieldless").contains("TestStorage"));
}
