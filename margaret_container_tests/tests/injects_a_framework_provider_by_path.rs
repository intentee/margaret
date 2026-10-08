use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::tag::Tag;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::render_container::render_container;
use margaret_container::rendered_container::RenderedContainer;
use margaret_container::resolve_injectable::resolve_injectable;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn framework_provider(injection: FrameworkInjectionRole, name: &str) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::WhenReferenced,
        injection,
        provided: CanonicalPath::new(vec!["crate".to_string(), name.to_string()]),
    }
}

fn issuer_client() -> FrameworkProvider {
    let path: syn::Path = syn::parse_str("auth").expect("the tag path parses");

    framework_provider(
        FrameworkInjectionRole::TrustedIssuer(
            Tag::from_path(&path).expect("the tag is a plain name"),
        ),
        "IssuerClient",
    )
}

fn index(fixture: &str) -> AttributeIndex {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build()
}

fn render(
    index: &AttributeIndex,
    providers: &[FrameworkProvider],
) -> Result<RenderedContainer, ContainerError> {
    render_container(
        index,
        &scan(index).expect("the serve inputs are scanned"),
        providers,
        &DeclaredTokenIssuance::Absent,
    )
}

#[test]
fn injects_a_framework_provider_by_its_path() {
    let source: String = container_module_source(
        render(
            &index("framework_provider_by_path"),
            &[framework_provider(
                FrameworkInjectionRole::Unmarked,
                "FrameworkStore",
            )],
        )
        .expect("the framework provider resolves by its path")
        .modules,
    )
    .split_whitespace()
    .collect();

    assert!(source.contains("crate::Consumer::new(::std::sync::Arc::clone(&framework_store)),"));
}

#[test]
fn refuses_to_inject_a_token_issuer_client_into_a_singleton() {
    assert!(matches!(
        render(&index("token_issuer_client_by_path"), &[issuer_client()])
            .err()
            .expect("a token issuer client is reserved for the framework"),
        ContainerError::FrameworkOnlyProvider { .. }
    ));
}

#[test]
fn refuses_to_inject_a_token_issuer_client_into_a_request_site() {
    let index = index("framework_provider_by_path");
    let RenderedContainer { bindings, .. } = render(
        &index,
        &[
            framework_provider(FrameworkInjectionRole::Unmarked, "FrameworkStore"),
            FrameworkProvider {
                enablement: FrameworkEnablement::Declared,
                ..issuer_client()
            },
        ],
    )
    .expect("the container renders");
    let consumer = index
        .items()
        .iter()
        .find(|item| item.identifier() == "Consumer")
        .expect("the consumer is indexed");
    let declared: syn::Type = syn::parse_str("Arc<crate::IssuerClient>").expect("the type parses");

    assert!(matches!(
        resolve_injectable(&index, consumer, &declared, &bindings),
        InjectableResolution::FrameworkOnly
    ));
}
