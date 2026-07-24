use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_container_tests::resolve_full_fixture::resolve_full_fixture;

#[test]
fn resolves_a_concrete_dependency_to_its_provider_accessor() {
    let InjectableResolution::Resolved(InjectedDependency { field, .. }) =
        resolve_full_fixture("App", "Arc<Config>")
    else {
        panic!("a concrete dependency must resolve to a single concrete accessor");
    };

    assert_eq!(field, "config");
}

#[test]
fn reports_a_single_dependency_with_no_provider() {
    assert!(matches!(
        resolve_full_fixture("App", "Arc<Unprovided>"),
        InjectableResolution::MissingProvider
    ));
}

#[test]
fn reports_an_unsupported_parameter_shape() {
    assert!(matches!(
        resolve_full_fixture("App", "String"),
        InjectableResolution::UnsupportedShape
    ));
}
