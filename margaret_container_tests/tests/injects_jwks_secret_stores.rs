use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::tag::Tag;
use margaret_console_argument_codegen::scan::scan;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container::rendered_container::RenderedContainer;
use margaret_container_tests::container_module_source::container_module_source;

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

fn render(fixture: &str, providers: &[FrameworkProvider]) -> Result<String, ContainerError> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");

    render_container(&index, &registry, providers).map(|RenderedContainer { modules, .. }| {
        container_module_source(modules)
            .split_whitespace()
            .collect()
    })
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
            "crate::Consumer::new(self.server_store().await?,self.auth_verifier().await?,)"
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
