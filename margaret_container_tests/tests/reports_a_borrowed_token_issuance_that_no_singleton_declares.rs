use margaret_container::container_error::ContainerError;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container_tests::constructed_from::constructed_from;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;

#[test]
fn reports_a_borrowed_token_issuance_that_no_singleton_declares() {
    assert!(matches!(
        render_with_framework_providers(
            "token_issuance_absent",
            &[constructed_from(
                vec![FrameworkDependency::BorrowedTokenIssuance],
                "Catalog",
            )],
        ),
        Err(ContainerError::MissingTokenIssuance { ref provider }) if provider == "crate::Catalog"
    ));
}
