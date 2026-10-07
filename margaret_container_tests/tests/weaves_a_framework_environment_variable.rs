use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container::rendered_container::RenderedContainer;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn vault_provider() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![FrameworkDependency::EnvironmentVariable {
                name: EnvironmentVariableName::new("VAULT_TOKEN")
                    .expect("the variable name is well formed"),
                value_type: CanonicalPath::new(vec!["crate".to_string(), "VaultToken".to_string()]),
            }],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "Vault".to_string()]),
    }
}

fn render(fixture: &str) -> Result<RenderedContainer, ContainerError> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the serve inputs are scanned");

    render_container(
        &index,
        &registry,
        &[vault_provider()],
        &DeclaredTokenIssuance::Absent,
    )
}

#[test]
fn reads_one_variable_shared_by_the_framework_and_the_application() {
    let rendered = render("framework_environment_variable").expect("the fixture renders");

    assert_eq!(rendered.bindings.all_serve_inputs().len(), 1);
    assert_eq!(
        rendered.bindings.all_serve_inputs()[0].name(),
        "VAULT_TOKEN"
    );
}

#[test]
fn hands_the_variable_to_the_framework_constructor() {
    let source: String = container_module_source(
        render("framework_environment_variable")
            .expect("the fixture renders")
            .modules,
    )
    .split_whitespace()
    .collect();

    assert!(source.contains("crate::Vault::create(serve_input_0"));
}

#[test]
fn rejects_a_variable_the_application_reads_as_another_type() {
    assert!(
        render("framework_environment_variable_conflict")
            .err()
            .expect("the conflicting variable is rejected")
            .to_string()
            .contains("a shared serve input must be declared identically everywhere")
    );
}
