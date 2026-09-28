use std::path::Path;

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

fn auth_tag() -> Tag {
    let path: syn::Path = syn::parse_str("auth").expect("the tag path parses");

    Tag::from_path(&path).expect("the tag is a plain name")
}

fn store_provider(injection: FrameworkInjectionRole, name: &str) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::WhenReferenced,
        injection,
        provided: CanonicalPath::new(vec!["crate".to_string(), name.to_string()]),
    }
}

fn both_stores() -> [FrameworkProvider; 2] {
    [
        store_provider(FrameworkInjectionRole::JwksServerStore, "ServerStore"),
        store_provider(
            FrameworkInjectionRole::JwksClientStore(auth_tag()),
            "AuthVerifier",
        ),
    ]
}

fn render(fixture: &str, providers: &[FrameworkProvider]) -> Result<String, ContainerError> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");

    render_container(&index, &serve_inputs, providers).map(
        |RenderedContainer { modules, .. }| {
            container_module_source(modules)
                .split_whitespace()
                .collect()
        },
    )
}

#[test]
fn resolves_the_server_store_and_client_verifier_by_marker() {
    let source = render(
        "jwks_secret_stores",
        &[
            store_provider(FrameworkInjectionRole::JwksServerStore, "ServerStore"),
            store_provider(
                FrameworkInjectionRole::JwksClientStore(auth_tag()),
                "AuthVerifier",
            ),
        ],
    )
    .expect("the server and client stores resolve");

    assert!(
        source.contains(
            "crate::Consumer::new(::std::sync::Arc::clone(&server_store),::std::sync::Arc::clone(&auth_verifier),)"
        )
    );
}

#[test]
fn reports_an_unavailable_jwks_server_store() {
    let error =
        render("jwks_secret_stores", &[]).expect_err("the server store must be unavailable");

    assert!(error.to_string().contains("the server"));
}

#[test]
fn reports_an_unknown_jwks_client_store_tag() {
    let error = render(
        "jwks_secret_stores",
        &[store_provider(
            FrameworkInjectionRole::JwksServerStore,
            "ServerStore",
        )],
    )
    .expect_err("the client store tag must be unknown");

    assert!(error.to_string().contains("client 'auth'"));
}

#[test]
fn rejects_a_malformed_jwks_secret_store_marker() {
    let error =
        render("jwks_malformed_store", &[]).expect_err("the malformed marker must be rejected");

    assert!(
        error
            .to_string()
            .contains("either `server` or `client = <tag>`")
    );
}

#[test]
fn rejects_an_unparseable_jwks_secret_store_marker() {
    let error =
        render("jwks_unparseable_store", &[]).expect_err("the unparseable marker must be rejected");

    assert!(matches!(error, ContainerError::Index { .. }));
}

#[test]
fn propagates_an_unparseable_jwks_store_on_a_service_constructor() {
    let error = render("service_unparseable_jwks_store", &[])
        .expect_err("the unparseable service marker must be rejected");

    assert!(matches!(error, ContainerError::Index { .. }));
}

#[test]
fn rejects_a_parameter_that_is_both_a_serve_input_and_a_jwks_secret_store() {
    let error = render(
        "serve_input_and_jwks_secret_store",
        &[store_provider(
            FrameworkInjectionRole::JwksServerStore,
            "ServerStore",
        )],
    )
    .expect_err("a parameter resolves to exactly one source");

    assert!(
        error
            .to_string()
            .contains("carries a serve input together with #[jwks_secret_store]")
    );
}

#[test]
fn reports_a_marked_parameter_declared_as_another_type() {
    assert!(matches!(
        render("jwks_mismatched_store_type", &both_stores())
            .expect_err("a marked parameter must be declared as the provider it receives"),
        ContainerError::MismatchedJwksSecretStoreType { .. }
    ));
}

#[test]
fn reports_a_jwks_secret_store_injected_by_path() {
    assert!(matches!(
        render("jwks_store_by_path", &both_stores())
            .expect_err("a jwks secret store must be reachable only through its marker"),
        ContainerError::JwksSecretStoreInjectedByPath { .. }
    ));
}

#[test]
fn refuses_to_inject_a_jwks_secret_store_by_path_into_a_request_site() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/jwks_secret_stores");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");
    let RenderedContainer { bindings, .. } =
        render_container(&index, &serve_inputs, &both_stores()).expect("the container renders");
    let consumer = index
        .items()
        .iter()
        .find(|item| item.identifier() == "Consumer")
        .expect("the consumer is indexed");
    let declared: syn::Type = syn::parse_str("Arc<ServerStore>").expect("the type parses");

    assert!(matches!(
        resolve_injectable(&index, consumer, &declared, &bindings),
        InjectableResolution::JwksSecretStoreByPath
    ));
}
