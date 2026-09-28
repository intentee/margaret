use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_serve_input_codegen::scan::scan;

fn jwks_client_provider() -> FrameworkProvider {
    let endpoint = CanonicalPath::new(vec!["crate".to_string(), "JwksEndpoint".to_string()]);

    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![
                FrameworkDependency::SingletonView(endpoint.clone()),
                FrameworkDependency::SingletonView(endpoint),
            ],
            is_async: true,
            method: "new".to_string(),
        },
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "JwksClientRuntime".to_string()]),
    }
}

fn endpoints_container() -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoints");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the endpoint fixture is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");

    container_module_source(
        render_container(&index, &serve_inputs, &[jwks_client_provider()])
            .expect("the endpoint fixture renders")
            .modules,
    )
    .split_whitespace()
    .collect()
}

#[test]
fn stores_the_jwks_endpoint_as_its_concrete_singleton() {
    assert!(endpoints_container().contains(
        "letjwks_endpoint=margaret::framework::construct_singleton::construct_singleton(\"crate::JwksEndpoint\""
    ));
}

#[test]
fn injects_the_endpoints_own_dependencies_and_console_argument() {
    assert!(endpoints_container().contains(
        "crate::JwksEndpoint::new(::std::sync::Arc::clone(&dns_resolver),serve_input_0)"
    ));
}

#[test]
fn injects_the_endpoint_into_the_framework_client_by_tag() {
    let source = endpoints_container();

    assert!(source.contains(
        "crate::JwksClientRuntime::new(::std::sync::Arc::<crate::JwksEndpoint>::clone(&jwks_endpoint),::std::sync::Arc::<crate::JwksEndpoint>::clone(&jwks_endpoint),).await"
    ));
}

#[test]
fn constructs_the_endpoint_wrapped_in_an_arc() {
    let source = endpoints_container();

    assert!(source.contains("::std::sync::Arc::new("));
    assert!(
        source.contains("construct_singleton(\"crate::JwksEndpoint\",crate::JwksEndpoint::new")
    );
}
