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

#[test]
fn reports_a_framework_provider_colliding_with_a_user_singleton() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/framework_provider_collision");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");
    let framework_providers = vec![FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "Widget".to_string()]),
    }];

    let error = render_container(
        &index,
        &registry,
        &framework_providers,
        &DeclaredTokenIssuance::Absent,
    )
    .err()
    .expect("a framework provider colliding with a user singleton must be rejected");

    assert!(matches!(
        error,
        ContainerError::AmbiguousFrameworkProvider { .. }
    ));
}
