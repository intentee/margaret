use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn provider_of_an_undeclared_dependency() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Accessor {
            accessor: "unavailable".to_string(),
            source: CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()]),
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "Accessed".to_string()]),
    }
}

#[test]
fn rejects_a_framework_provider_whose_dependency_is_not_declared() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/framework_provider_undeclared_dependency");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");

    let error = render_container(
        &index,
        &registry,
        &[provider_of_an_undeclared_dependency()],
        &DeclaredTokenIssuance::Absent,
    )
    .err()
    .expect("a framework provider with an undeclared dependency is rejected");

    assert!(matches!(
        error,
        ContainerError::UndeclaredFrameworkDependency { provider, dependency }
            if provider == "crate::Accessed" && dependency == "crate::Missing"
    ));
}
