use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_container_tests::resolve_full_fixture::resolve_full_fixture;

#[test]
fn resolves_an_interface_dependency_to_its_provider_accessor() {
    let InjectableResolution::Resolved(InjectedDependency::SingleInterface { field, .. }) =
        resolve_full_fixture("App", "Arc<dyn Greeter>")
    else {
        panic!("an interface dependency must resolve to a single interface accessor");
    };

    assert_eq!(field, "english_greeter");
}

#[test]
fn resolves_a_concrete_dependency_to_its_provider_accessor() {
    let InjectableResolution::Resolved(InjectedDependency::SingleConcrete { field, .. }) =
        resolve_full_fixture("App", "Arc<Config>")
    else {
        panic!("a concrete dependency must resolve to a single concrete accessor");
    };

    assert_eq!(field, "config");
}

#[test]
fn resolves_a_collection_dependency_to_its_member_accessors() {
    let InjectableResolution::Resolved(InjectedDependency::Collection { member_fields, .. }) =
        resolve_full_fixture("App", "Vec<Arc<dyn Plugin>>")
    else {
        panic!("a collection dependency must resolve to member accessors");
    };

    assert_eq!(member_fields, vec!["logging_plugin", "metrics_plugin"]);
}

#[test]
fn resolves_an_empty_collection_dependency_to_no_accessors() {
    let InjectableResolution::Resolved(InjectedDependency::Collection { member_fields, .. }) =
        resolve_full_fixture("App", "Vec<Arc<dyn Hook>>")
    else {
        panic!("a collection dependency without members must resolve to an empty accessor list");
    };

    assert!(member_fields.is_empty());
}

#[test]
fn reports_a_single_dependency_with_no_provider() {
    assert!(matches!(
        resolve_full_fixture("App", "Arc<EnglishGreeter>"),
        InjectableResolution::MissingProvider
    ));
}

#[test]
fn reports_a_collection_dependency_with_no_indexed_trait() {
    assert!(matches!(
        resolve_full_fixture("App", "Vec<Arc<dyn Unknown>>"),
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
