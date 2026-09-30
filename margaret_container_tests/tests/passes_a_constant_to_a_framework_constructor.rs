use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container_tests::constructed_from::constructed_from;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_container_tests::crate_path::crate_path;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;

#[test]
fn passes_a_constant_to_a_framework_constructor() {
    let source: String = container_module_source(
        render_with_framework_providers(
            "token_issuance",
            &[constructed_from(
                vec![FrameworkDependency::Constant(crate_path("ROUTE_PATHS"))],
                "Endpoints",
            )],
        )
        .expect("the constant fixture renders")
        .modules,
    )
    .split_whitespace()
    .collect();

    assert!(source.contains("crate::Endpoints::create(crate::ROUTE_PATHS)"));
}
