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
use margaret_container_tests::container_module_source::container_module_source;
use margaret_database_codegen::database_canonical_path::database_canonical_path;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn database() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::Declared,
        injection: FrameworkInjectionRole::Unmarked,
        provided: database_canonical_path(),
    }
}

fn key_roller() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![FrameworkDependency::Database],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "KeyRoller".to_string()]),
    }
}

fn rendered(fixture: &str) -> Result<String, ContainerError> {
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
        &[database(), key_roller()],
        &DeclaredPostgresDatabase::read(&index).expect("the database declaration is read"),
        &DeclaredTokenIssuance::Absent,
    )
    .map(|rendered| {
        container_module_source(rendered.modules)
            .split_whitespace()
            .collect()
    })
}

#[test]
fn hands_the_declared_database_to_the_framework_constructor() {
    assert!(rendered("declared_database").expect("the fixture renders").contains(
        "crate::KeyRoller::create(::std::sync::Arc::<margaret::framework::database::database::Database,>::clone("
    ));
}

#[test]
fn reports_a_framework_provider_without_a_declared_database() {
    assert!(matches!(
        rendered("undeclared_database"),
        Err(ContainerError::MissingPostgresDatabase { provider }) if provider == "crate::KeyRoller"
    ));
}
