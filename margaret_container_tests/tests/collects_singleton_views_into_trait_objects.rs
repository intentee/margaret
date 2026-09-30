use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container_tests::constructed_from::constructed_from;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_container_tests::crate_path::crate_path;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;

#[test]
fn collects_singleton_views_into_trait_objects() {
    let source: String = container_module_source(
        render_with_framework_providers(
            "endpoints",
            &[constructed_from(
                vec![FrameworkDependency::SingletonViews {
                    singletons: vec![crate_path("DnsResolver"), crate_path("JwksEndpoint")],
                    view: CanonicalPath::new(vec!["crate".to_string(), "Declares".to_string()]),
                }],
                "Registry",
            )],
        )
        .expect("the viewing fixture renders")
        .modules,
    )
    .split_whitespace()
    .collect();

    assert!(source.contains(
        "crate::Registry::create(::std::vec::Vec::from([::std::sync::Arc::<crate::DnsResolver>::clone(&dns_resolver)as::std::sync::Arc<dyncrate::Declares>,::std::sync::Arc::<crate::JwksEndpoint>::clone(&jwks_endpoint)as::std::sync::Arc<dyncrate::Declares>,]),),)"
    ));
}
