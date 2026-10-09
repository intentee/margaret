use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oauth_client_codegen::declared_client_authentication::DeclaredClientAuthentication;
use margaret_oauth_client_codegen::oauth_client_credentials::OAuthClientCredentials;
use margaret_oauth_client_codegen::oauth_client_id_path::oauth_client_id_path;
use margaret_oauth_client_codegen::oauth_client_item::OAuthClientItem;
use margaret_oauth_client_codegen::oauth_client_item_path::oauth_client_item_path;
use margaret_oauth_client_codegen::oauth_client_resource_credentials_path::oauth_client_resource_credentials_path;
use margaret_oauth_client_codegen::oauth_client_resource_grant_path::oauth_client_resource_grant_path;
use margaret_oauth_client_codegen::oauth_client_sign_in_callback_handler_path::oauth_client_sign_in_callback_handler_path;
use margaret_oauth_client_codegen::oauth_client_sign_in_scopes_path::oauth_client_sign_in_scopes_path;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_sessions_codegen::sessions_item::SessionsItem;
use margaret_sessions_codegen::sessions_item_path::sessions_item_path;
use margaret_sign_in_endpoints_codegen::client_sign_in::ClientSignIn;
use margaret_sign_in_endpoints_codegen::served_sign_in::ServedSignIn;
use margaret_sign_in_endpoints_codegen::sign_in_callback::SignInCallback;
use margaret_sign_in_endpoints_codegen::sign_in_service::SignInService;
use margaret_tag_codegen::bound_authorization_server::BoundAuthorizationServer;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

use crate::issuer_request_client_canonical_path::issuer_request_client_canonical_path;
use crate::jwks_secret_holder_canonical_path::jwks_secret_holder_canonical_path;

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

fn external_construction(
    tag: &Tag,
    authentication: &DeclaredClientAuthentication,
    TrustedIssuerBinding { group, trust }: &TrustedIssuerBinding,
) -> FrameworkConstruction {
    let dependencies = |credential| {
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
        ]
    };

    match authentication {
        DeclaredClientAuthentication::ClientSecretBasic { client_secret_from } => created(
            dependencies(FrameworkDependency::EnvironmentVariable {
                name: client_secret_from.clone(),
                value_type: client_secret_path(),
            }),
            "with_client_secret_basic",
        ),
        DeclaredClientAuthentication::PrivateKeyJwt => created(
            dependencies(FrameworkDependency::Provider(
                jwks_secret_holder_canonical_path(),
            )),
            "with_private_key_jwt",
        ),
    }
}

fn own_construction(tag: &Tag) -> FrameworkConstruction {
    created(
        vec![
            FrameworkDependency::Provider(issuer_request_client_canonical_path()),
            FrameworkDependency::Provider(oidc_provider_item_path(
                OidcProviderItem::IssuerMetadata,
            )),
            FrameworkDependency::Provider(trusted_issuer_item_path(
                tag,
                TrustedIssuerItem::TrustedIssuer,
            )),
            FrameworkDependency::Constant(oauth_client_id_path(tag)),
            FrameworkDependency::Provider(jwks_secret_holder_canonical_path()),
        ],
        "with_private_key_jwt",
    )
}

fn authorization_server_construction(
    OAuthClientBinding { client, server, .. }: &OAuthClientBinding,
) -> FrameworkConstruction {
    match server {
        BoundAuthorizationServer::External {
            authentication,
            issuer,
            ..
        } => external_construction(&client.tag, authentication, issuer),
        BoundAuthorizationServer::Own { .. } => own_construction(&client.tag),
    }
}

fn sign_in_providers(
    tag: &Tag,
    authorization_server: &CanonicalPath,
    ServedSignIn {
        admission,
        callback: SignInCallback { landing, url, .. },
        ..
    }: &ServedSignIn,
) -> Vec<FrameworkProvider> {
    let flow = oauth_client_item_path(tag, OAuthClientItem::SignInFlow);
    let framework_state = |construction, enablement, provided| FrameworkProvider {
        construction,
        enablement,
        injection: FrameworkInjectionRole::FrameworkState,
        provided,
    };

    vec![
        framework_state(
            FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Provider(authorization_server.clone()),
                    FrameworkDependency::Provider(jwks_secret_holder_canonical_path()),
                    FrameworkDependency::RouteUrl(url.clone()),
                    FrameworkDependency::Constant(oauth_client_sign_in_scopes_path(tag)),
                ],
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Fallible,
            },
            FrameworkEnablement::Dependency,
            flow.clone(),
        ),
        framework_state(
            created(vec![FrameworkDependency::Provider(flow.clone())], "create"),
            FrameworkEnablement::Declared,
            oauth_client_item_path(tag, OAuthClientItem::SignInStartHandler),
        ),
        framework_state(
            created(
                vec![
                    FrameworkDependency::Provider(flow),
                    FrameworkDependency::SingletonView(admission.clone()),
                    FrameworkDependency::Provider(sessions_item_path(SessionsItem::IssuedSessions)),
                    FrameworkDependency::RouteUrl(landing.clone()),
                ],
                "create",
            ),
            FrameworkEnablement::Declared,
            oauth_client_sign_in_callback_handler_path(tag),
        ),
    ]
}

fn oauth_client_providers(
    ClientSignIn { binding, service }: &ClientSignIn,
) -> Vec<FrameworkProvider> {
    let tag = &binding.client.tag;
    let authorization_server =
        oauth_client_item_path(tag, OAuthClientItem::AuthorizationServerClient);
    let depending_on_server = |item: OAuthClientItem| FrameworkProvider {
        construction: created(
            vec![FrameworkDependency::Provider(authorization_server.clone())],
            "create",
        ),
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: oauth_client_item_path(tag, item),
    };

    let mut providers = vec![
        FrameworkProvider {
            construction: authorization_server_construction(binding),
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::OAuthClient(tag.clone()),
            provided: authorization_server.clone(),
        },
        depending_on_server(OAuthClientItem::TokenExchange),
    ];

    match &binding.credentials {
        OAuthClientCredentials::PerResource { resources, .. } => {
            providers.extend(resources.iter().map(|resource| FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![
                        FrameworkDependency::Provider(authorization_server.clone()),
                        FrameworkDependency::Constant(oauth_client_resource_grant_path(
                            tag, resource,
                        )),
                    ],
                    is_async: false,
                    method: "create".to_string(),
                    outcome: ConstructorOutcome::Fallible,
                },
                enablement: FrameworkEnablement::WhenReferenced,
                injection: FrameworkInjectionRole::Unmarked,
                provided: oauth_client_resource_credentials_path(tag, resource),
            }));
        }
        OAuthClientCredentials::Targeted => {
            providers.push(depending_on_server(OAuthClientItem::ClientCredentials));
        }
        OAuthClientCredentials::Withheld => {}
    }

    if let SignInService::Served(served) = service {
        providers.extend(sign_in_providers(tag, &authorization_server, served));
    }

    providers
}

pub(crate) fn oauth_client_framework_providers(
    client_sign_ins: &[ClientSignIn],
) -> Vec<FrameworkProvider> {
    client_sign_ins
        .iter()
        .flat_map(oauth_client_providers)
        .collect()
}
