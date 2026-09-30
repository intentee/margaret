use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container_tests::constructed_from::constructed_from;
use margaret_container_tests::crate_path::crate_path;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;
use margaret_container_tests::unit_dependency::unit_dependency;

#[test]
fn reports_the_direct_dependencies_of_a_provider() {
    let rendered = render_with_framework_providers(
        "token_issuance",
        &[
            unit_dependency("Pool"),
            constructed_from(
                vec![FrameworkDependency::Provider(crate_path("Pool"))],
                "Catalog",
            ),
        ],
    )
    .expect("the dependent fixture renders");

    assert!(
        rendered
            .bindings
            .depends_directly_on(&crate_path("Catalog"), &crate_path("Pool"))
    );
    assert!(
        !rendered
            .bindings
            .depends_directly_on(&crate_path("Pool"), &crate_path("Catalog"))
    );
    assert!(
        !rendered
            .bindings
            .depends_directly_on(&crate_path("Unplanned"), &crate_path("Pool"))
    );
}
