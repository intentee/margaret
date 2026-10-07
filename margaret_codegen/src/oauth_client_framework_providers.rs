use std::iter;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oauth_client_codegen::declared_client_authentication::DeclaredClientAuthentication;
use margaret_oauth_client_codegen::oauth_client_declaration::OAuthClientDeclaration;
use margaret_oauth_client_codegen::oauth_client_id_path::oauth_client_id_path;
use margaret_oauth_client_codegen::oauth_client_item::OAuthClientItem;
use margaret_oauth_client_codegen::oauth_client_item_path::oauth_client_item_path;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

use crate::issuer_request_client_canonical_path::issuer_request_client_canonical_path;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;

fn created(dependencies: Vec<FrameworkDependency>, method: &str) -> FrameworkConstruction {
    FrameworkConstruction::Constructor {
        dependencies,
        is_async: false,
        method: method.to_string(),
        outcome: ConstructorOutcome::Infallible,
    }
}

fn client_secret_path() -> CanonicalPath {
    CanonicalPath::new(
        [
            "margaret",
            "framework",
            "oauth_vocabulary",
            "client_secret",
            "ClientSecret",
        ]
        .iter()
        .map(ToString::to_string)
        .collect(),
    )
}

fn authorization_server_construction(
    OAuthClientDeclaration {
        authentication,
        tag,
        ..
    }: &OAuthClientDeclaration,
    TrustedIssuerBinding { group, trust }: &TrustedIssuerBinding,
) -> FrameworkConstruction {
    let (credential, method) = match authentication {
        DeclaredClientAuthentication::ClientSecretBasic { client_secret_from } => (
            FrameworkDependency::EnvironmentVariable {
                name: client_secret_from.clone(),
                value_type: client_secret_path(),
            },
            "with_client_secret_basic",
        ),
        DeclaredClientAuthentication::PrivateKeyJwt => (
            FrameworkDependency::Provider(jwks_roller_canonical_path()),
            "with_private_key_jwt",
        ),
    };

    created(
        vec![
            FrameworkDependency::Provider(issuer_request_client_canonical_path()),
            FrameworkDependency::Provider(trusted_issuer_item_path(
                &group.lead().tag,
                TrustedIssuerItem::IssuerMetadata,
            )),
            FrameworkDependency::Provider(trusted_issuer_item_path(
                &trust.tag,
                TrustedIssuerItem::TrustedIssuer,
            )),
            FrameworkDependency::Constant(oauth_client_id_path(tag)),
            credential,
        ],
        method,
    )
}

fn oauth_client_providers(
    OAuthClientBinding { client, issuer }: &OAuthClientBinding,
) -> [FrameworkProvider; 4] {
    let tag = &client.tag;
    let authorization_server =
        oauth_client_item_path(tag, OAuthClientItem::AuthorizationServerClient);
    let depending_on_server =
        |item: OAuthClientItem, dependencies: Vec<FrameworkDependency>| FrameworkProvider {
            construction: created(
                iter::once(FrameworkDependency::Provider(authorization_server.clone()))
                    .chain(dependencies)
                    .collect(),
                "create",
            ),
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: oauth_client_item_path(tag, item),
        };

    [
        FrameworkProvider {
            construction: authorization_server_construction(client, issuer),
            enablement: FrameworkEnablement::Always,
            injection: FrameworkInjectionRole::OAuthClient(tag.clone()),
            provided: authorization_server.clone(),
        },
        depending_on_server(OAuthClientItem::ClientCredentials, Vec::new()),
        depending_on_server(
            OAuthClientItem::SignInFlow,
            vec![FrameworkDependency::Provider(jwks_roller_canonical_path())],
        ),
        depending_on_server(OAuthClientItem::TokenExchange, Vec::new()),
    ]
}

pub(crate) fn oauth_client_framework_providers(
    oauth_client_bindings: &[OAuthClientBinding],
) -> Vec<FrameworkProvider> {
    oauth_client_bindings
        .iter()
        .flat_map(oauth_client_providers)
        .collect()
}
