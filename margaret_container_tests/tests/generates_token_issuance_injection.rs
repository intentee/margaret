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
use margaret_container::preparation_source::PreparationSource;
use margaret_container::render_container::render_container;
use margaret_container::rendered_container::RenderedContainer;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn issued_token_store() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![FrameworkDependency::TokenIssuance],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::Declared,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "IssuedTokenStore".to_string()]),
    }
}

fn prepared_token_store() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Prepared {
            method: "create".to_string(),
            sources: vec![PreparationSource::TokenIssuance],
        },
        enablement: FrameworkEnablement::Declared,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "PreparedTokenStore".to_string()]),
    }
}

fn render(fixture: &str, provider: FrameworkProvider) -> Result<RenderedContainer, ContainerError> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");

    render_container(
        &index,
        &serve_inputs,
        &[provider],
        &DeclaredPostgresDatabase::Absent,
        &DeclaredTokenIssuance::read(&index).expect("the token issuance is read"),
    )
}

#[test]
fn passes_the_declared_token_issuance_constant_so_its_one_declaration_is_reused() {
    let source: String = container_module_source(
        render("token_issuance", issued_token_store())
            .expect("the issuance fixture renders")
            .modules,
    )
    .split_whitespace()
    .collect();

    assert!(source.contains(
        "crate::IssuedTokenStore::create(crate::margaret::token_issuance::TOKEN_ISSUANCE)"
    ));
}

#[test]
fn rejects_a_token_issuance_without_a_declared_issuer() {
    assert!(matches!(
        render("token_issuance_absent", issued_token_store()),
        Err(ContainerError::MissingTokenIssuance { .. })
    ));
}

#[test]
fn prepares_a_value_from_the_declared_token_issuance_constant() {
    let rendered =
        render("token_issuance", prepared_token_store()).expect("the issuance fixture renders");

    assert_eq!(
        rendered
            .bindings
            .serve_inputs(&[CanonicalPath::new(vec![
                "crate".to_string(),
                "PreparedTokenStore".to_string(),
            ])])
            .expect("the store has planned serve inputs")
            .iter()
            .map(|slotted| slotted.input.name())
            .collect::<Vec<String>>(),
        ["crate::PreparedTokenStore::create"]
    );
}

#[test]
fn rejects_a_preparation_from_a_token_issuance_without_a_declared_issuer() {
    assert!(matches!(
        render("token_issuance_absent", prepared_token_store()),
        Err(ContainerError::MissingTokenIssuance { provider })
            if provider == "crate::PreparedTokenStore"
    ));
}
