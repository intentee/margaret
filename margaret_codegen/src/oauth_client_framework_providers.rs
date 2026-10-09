use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use margaret_oauth_client_codegen::declared_client_authentication::DeclaredClientAuthentication;
use margaret_oauth_client_codegen::oauth_client_id_path::oauth_client_id_path;
use margaret_oauth_client_codegen::oauth_client_item::OAuthClientItem;
use margaret_oauth_client_codegen::oauth_client_item_path::oauth_client_item_path;
use margaret_oauth_client_codegen::oauth_client_sign_in_scopes_path::oauth_client_sign_in_scopes_path;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_tag_codegen::bound_authorization_server::BoundAuthorizationServer;
use margaret_tag_codegen::bound_sign_in::BoundSignIn;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;
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
            dependencies(FrameworkDependency::Provider(jwks_roller_canonical_path())),
            "with_private_key_jwt",
        ),
    }
}

fn own_construction(tag: &Tag, issuance: &TokenIssuanceDeclaration) -> FrameworkConstruction {
    created(
        vec![
            FrameworkDependency::Provider(issuer_request_client_canonical_path()),
            FrameworkDependency::Provider(oidc_provider_item_path(
                OidcProviderItem::IssuerMetadata,
            )),
            FrameworkDependency::Provider(trusted_issuer_item_path(
                &issuance.tag,
                TrustedIssuerItem::TrustedIssuer,
            )),
            FrameworkDependency::Constant(oauth_client_id_path(tag)),
            FrameworkDependency::Provider(jwks_roller_canonical_path()),
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
        BoundAuthorizationServer::Own { issuance, .. } => own_construction(&client.tag, issuance),
    }
}

fn sign_in_flow_providers(
    binding: &OAuthClientBinding,
    authorization_server: &CanonicalPath,
    routes: &DeclaredRoutes,
) -> Result<Vec<FrameworkProvider>, HttpCodegenError> {
    let BoundSignIn::Available { redirect_route, .. } = binding.sign_in else {
        return Ok(Vec::new());
    };
    let tag = &binding.client.tag;

    Ok(vec![FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![
                FrameworkDependency::Provider(authorization_server.clone()),
                FrameworkDependency::Provider(jwks_roller_canonical_path()),
                FrameworkDependency::RouteUrl(
                    routes
                        .redirect_target(redirect_route, binding.client.anchor.canonical_path())?,
                ),
                FrameworkDependency::Constant(oauth_client_sign_in_scopes_path(tag)),
            ],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Fallible,
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: oauth_client_item_path(tag, OAuthClientItem::SignInFlow),
    }])
}

fn oauth_client_providers(
    binding: &OAuthClientBinding,
    routes: &DeclaredRoutes,
) -> Result<Vec<FrameworkProvider>, HttpCodegenError> {
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

    Ok([
        FrameworkProvider {
            construction: authorization_server_construction(binding),
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::OAuthClient(tag.clone()),
            provided: authorization_server.clone(),
        },
        depending_on_server(OAuthClientItem::ClientCredentials),
        depending_on_server(OAuthClientItem::TokenExchange),
    ]
    .into_iter()
    .chain(sign_in_flow_providers(
        binding,
        &authorization_server,
        routes,
    )?)
    .collect())
}

pub(crate) fn oauth_client_framework_providers(
    oauth_client_bindings: &[OAuthClientBinding],
    routes: &DeclaredRoutes,
) -> Result<Vec<FrameworkProvider>, HttpCodegenError> {
    let mut providers = Vec::new();

    for binding in oauth_client_bindings {
        providers.extend(oauth_client_providers(binding, routes)?);
    }

    Ok(providers)
}
