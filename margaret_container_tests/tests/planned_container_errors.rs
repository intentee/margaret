use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::plan_container::plan_container;
use margaret_container::planned_container::PlannedContainer;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn planned_container() -> PlannedContainer {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");
    plan_container(
        &index,
        &registry,
        &[],
        &DeclaredPostgresDatabase::Absent,
        &DeclaredTokenIssuance::Absent,
    )
    .expect("the container is planned")
}

fn missing_path() -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()])
}

fn planned_with(fixture: &str, framework_provider: FrameworkProvider) -> PlannedContainer {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");

    plan_container(
        &index,
        &registry,
        &[framework_provider],
        &DeclaredPostgresDatabase::Absent,
        &DeclaredTokenIssuance::Absent,
    )
    .expect("the container is planned")
}

fn unit_provider_of(name: &str, enablement: FrameworkEnablement) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), name.to_string()]),
    }
}

fn unit_provider(enablement: FrameworkEnablement) -> FrameworkProvider {
    unit_provider_of("Framework", enablement)
}

fn user_roots(planned: &PlannedContainer) -> Vec<CanonicalPath> {
    planned
        .roots()
        .into_iter()
        .filter(|root| root != &unit_provider(FrameworkEnablement::Dependency).provided)
        .collect()
}

#[test]
fn reports_an_unplanned_served_root() {
    let planned = planned_container();
    let Err(error) = planned.render(&[missing_path()], &[]) else {
        panic!("an unplanned served root must not be rendered");
    };

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unplanned_builder_root() {
    let planned = planned_container();
    let roots = planned.roots();
    let Err(error) = planned.render(&roots, &[missing_path()]) else {
        panic!("an unplanned builder root must not be rendered");
    };

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn rejects_a_declared_framework_provider_no_root_reaches() {
    let planned = planned_with(
        "serve_input_propagation",
        unit_provider(FrameworkEnablement::Declared),
    );
    let roots = user_roots(&planned);

    assert!(matches!(
        planned.render(&roots, &[]),
        Err(ContainerError::UnconsumedDeclaration { path }) if path == "crate::Framework"
    ));
}

#[test]
fn builds_no_optional_framework_provider_no_root_reaches() {
    let planned = planned_with(
        "serve_input_propagation",
        unit_provider(FrameworkEnablement::Dependency),
    );
    let roots = user_roots(&planned);

    assert!(
        !container_module_source(
            planned
                .render(&roots, &[])
                .expect("the container renders")
                .modules
        )
        .contains("crate::Framework")
    );
}

#[test]
fn names_the_unconsumed_singleton_rather_than_the_framework_provider_it_depends_on() {
    let planned = planned_with(
        "framework_provider_by_path",
        unit_provider_of("FrameworkStore", FrameworkEnablement::WhenReferenced),
    );

    assert!(matches!(
        planned.render(&[], &[]),
        Err(ContainerError::UnconsumedSingleton { path }) if path == "crate::Consumer"
    ));
}
