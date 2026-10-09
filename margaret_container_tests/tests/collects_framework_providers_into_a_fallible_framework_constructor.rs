use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn path(name: &str) -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
}

fn member(name: &str) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::Dependency,
        injection: FrameworkInjectionRole::Unmarked,
        provided: path(name),
    }
}

fn container_source() -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/viewed_singleton");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the viewed singleton fixture is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");
    let directory_provider = FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![FrameworkDependency::Providers(vec![
                path("FirstMember"),
                path("SecondMember"),
            ])],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Fallible,
        },
        enablement: FrameworkEnablement::Declared,
        injection: FrameworkInjectionRole::Unmarked,
        provided: path("Directory"),
    };

    container_module_source(
        render_container(
            &index,
            &serve_inputs,
            &[
                member("FirstMember"),
                member("SecondMember"),
                directory_provider,
            ],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the directory renders")
        .modules,
    )
    .split_whitespace()
    .collect()
}

#[test]
fn hands_every_collected_provider_to_the_constructor_in_order() {
    assert!(container_source().contains(
        "crate::Directory::create(::std::vec::Vec::from([::std::sync::Arc::<crate::FirstMember>::clone(&first_member),::std::sync::Arc::<crate::SecondMember>::clone(&second_member),]),)"
    ));
}

#[test]
fn propagates_the_failure_of_a_fallible_framework_constructor() {
    assert!(container_source().contains(
        "letdirectory=margaret::framework::construct_singleton::construct_singleton(\"crate::Directory\",crate::Directory::create(::std::vec::Vec::from([::std::sync::Arc::<crate::FirstMember>::clone(&first_member),::std::sync::Arc::<crate::SecondMember>::clone(&second_member),]),).map_err(margaret::framework::anyhow::Error::from),)?;"
    ));
}
