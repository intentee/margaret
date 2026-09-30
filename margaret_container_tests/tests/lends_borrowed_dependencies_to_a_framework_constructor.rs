use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container_tests::constructed_from::constructed_from;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_container_tests::crate_path::crate_path;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;
use margaret_container_tests::unit_dependency::unit_dependency;

#[test]
fn lends_borrowed_dependencies_to_a_framework_constructor() {
    let source: String = container_module_source(
        render_with_framework_providers(
            "token_issuance",
            &[
                unit_dependency("Pool"),
                constructed_from(
                    vec![
                        FrameworkDependency::BorrowedTokenIssuance,
                        FrameworkDependency::BorrowedProvider(crate_path("Pool")),
                    ],
                    "Catalog",
                ),
            ],
        )
        .expect("the borrowing fixture renders")
        .modules,
    )
    .split_whitespace()
    .collect();

    assert!(source.contains(
        "crate::Catalog::create(::std::sync::Arc::<crate::Issuer>::as_ref(&issuer),::std::sync::Arc::<crate::Pool>::as_ref(&pool),)"
    ));
}
