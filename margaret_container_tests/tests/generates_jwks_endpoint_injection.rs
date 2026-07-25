use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_argument_codegen::scan::scan;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container_tests::container_module_source::container_module_source;

fn jwks_client_provider() -> FrameworkProvider {
    let endpoint = CanonicalPath::new(vec!["crate".to_string(), "JwksEndpoint".to_string()]);

    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![FrameworkDependency::Endpoint(endpoint)],
            is_async: false,
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
    let registry = scan(&index).expect("the console arguments are scanned");

    container_module_source(
        render_container(&index, &registry, &[jwks_client_provider()])
            .expect("the endpoint fixture renders")
            .modules,
    )
    .split_whitespace()
    .collect()
}

#[test]
fn drafts_a_jwks_endpoint_as_the_provides_endpoint_interface() {
    assert!(
        endpoints_container()
            .contains("std::sync::Arc<dynmargaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint>")
    );
}

#[test]
fn injects_the_endpoints_own_dependencies_and_console_argument() {
    assert!(
        endpoints_container()
            .contains("crate::JwksEndpoint::new(self.dns_resolver().await,console_argument_0,)")
    );
}

#[test]
fn injects_the_endpoint_into_the_framework_client_by_tag() {
    assert!(endpoints_container().contains(
        "crate::JwksClientRuntime::new(self.jwks_endpoint(console_argument_0).await,)"
    ));
}

#[test]
fn constructs_the_endpoint_wrapped_in_an_arc() {
    assert!(endpoints_container().contains("=std::sync::Arc::new(crate::JwksEndpoint::new"));
}
