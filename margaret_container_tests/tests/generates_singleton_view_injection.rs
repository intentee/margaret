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
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn catalog_runtime_provider() -> FrameworkProvider {
    let catalog = CanonicalPath::new(vec!["crate".to_string(), "Catalog".to_string()]);

    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![
                FrameworkDependency::SingletonView(catalog.clone()),
                FrameworkDependency::SingletonView(catalog),
            ],
            is_async: true,
            method: "new".to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "CatalogRuntime".to_string()]),
    }
}

fn viewed_singleton_container() -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/viewed_singleton");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the viewed singleton fixture is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");

    container_module_source(
        render_container(
            &index,
            &serve_inputs,
            &[catalog_runtime_provider()],
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the viewed singleton fixture renders")
        .modules,
    )
    .split_whitespace()
    .collect()
}

#[test]
fn stores_the_viewed_singleton_as_its_concrete_singleton() {
    assert!(viewed_singleton_container().contains(
        "letcatalog=margaret::framework::construct_singleton::construct_singleton(\"crate::Catalog\""
    ));
}

#[test]
fn injects_the_viewed_singletons_own_dependencies_and_console_argument() {
    assert!(
        viewed_singleton_container()
            .contains("crate::Catalog::new(::std::sync::Arc::clone(&dns_resolver),serve_input_0)")
    );
}

#[test]
fn injects_the_viewed_singleton_into_the_framework_provider() {
    let source = viewed_singleton_container();

    assert!(source.contains(
        "crate::CatalogRuntime::new(::std::sync::Arc::<crate::Catalog>::clone(&catalog),::std::sync::Arc::<crate::Catalog>::clone(&catalog),).await"
    ));
}

#[test]
fn constructs_the_viewed_singleton_wrapped_in_an_arc() {
    let source = viewed_singleton_container();

    assert!(source.contains("::std::sync::Arc::new("));
    assert!(source.contains("construct_singleton(\"crate::Catalog\",crate::Catalog::new"));
}
