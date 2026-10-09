use margaret_accepted_clients_codegen::provider_aggregate::ProviderAggregate;
use margaret_accepted_clients_codegen::provider_aggregate_path::provider_aggregate_path;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oidc_provider_codegen::authorization_endpoint_url_path::authorization_endpoint_url_path;
use margaret_oidc_provider_codegen::authorization_handler_path::authorization_handler_path;
use margaret_oidc_provider_codegen::consent_page::ConsentPage;
use margaret_oidc_provider_codegen::declared_endpoint_routes::DeclaredEndpointRoutes;
use margaret_oidc_provider_codegen::derived_authorization::DerivedAuthorization;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;
use margaret_oidc_provider_codegen::provider_endpoints_path::provider_endpoints_path;
use margaret_oidc_provider_codegen::served_authorization::ServedAuthorization;
use margaret_oidc_provider_codegen::subject_token_exchanger_path::subject_token_exchanger_path;
use margaret_oidc_provider_codegen::userinfo_provision::UserinfoProvision;
use margaret_sessions_codegen::sessions_item::SessionsItem;
use margaret_sessions_codegen::sessions_item_path::sessions_item_path;
use margaret_tag_codegen::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

use crate::authorization_grants_tables_canonical_path::authorization_grants_tables_canonical_path;
use crate::served_endpoints::ServedEndpoints;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

fn grants_database() -> FrameworkDependency {
    FrameworkDependency::Database {
        framework_tables: vec![authorization_grants_tables_canonical_path()],
    }
}

fn constructed(
    dependencies: Vec<FrameworkDependency>,
    outcome: ConstructorOutcome,
    enablement: FrameworkEnablement,
    provided: CanonicalPath,
) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: "create".to_string(),
            outcome,
        },
        enablement,
        injection: FrameworkInjectionRole::Unmarked,
        provided,
    }
}

fn framework_state(
    dependencies: Vec<FrameworkDependency>,
    outcome: ConstructorOutcome,
    enablement: FrameworkEnablement,
    provided: CanonicalPath,
) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: "create".to_string(),
            outcome,
        },
        enablement,
        injection: FrameworkInjectionRole::FrameworkState,
        provided,
    }
}

fn served_endpoint(
    dependencies: Vec<FrameworkDependency>,
    outcome: ConstructorOutcome,
    provided: OidcProviderItem,
) -> FrameworkProvider {
    framework_state(
        dependencies,
        outcome,
        FrameworkEnablement::Declared,
        oidc_provider_item_path(provided),
    )
}

fn authorization_providers(
    ServedAuthorization {
        consent: ConsentPage { decision, view },
        ..
    }: &ServedAuthorization,
) -> Vec<FrameworkProvider> {
    let issued_sessions =
        || FrameworkDependency::Provider(sessions_item_path(SessionsItem::IssuedSessions));

    vec![
        framework_state(
            vec![
                item(OidcProviderItem::AcceptedClients),
                FrameworkDependency::Constant(authorization_endpoint_url_path()),
                FrameworkDependency::TokenIssuance,
            ],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::AuthorizationEndpoint),
        ),
        framework_state(
            vec![grants_database(), FrameworkDependency::TokenIssuance],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::ConsentEndpoint),
        ),
        framework_state(
            vec![
                item(OidcProviderItem::AuthorizationEndpoint),
                issued_sessions(),
                FrameworkDependency::SingletonView(view.clone()),
                FrameworkDependency::RouteUrl(decision.clone()),
                FrameworkDependency::Routes,
            ],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Declared,
            authorization_handler_path(),
        ),
        served_endpoint(
            vec![item(OidcProviderItem::ConsentEndpoint), issued_sessions()],
            ConstructorOutcome::Infallible,
            OidcProviderItem::ConsentHandler,
        ),
    ]
}

fn item(provided: OidcProviderItem) -> FrameworkDependency {
    FrameworkDependency::Provider(oidc_provider_item_path(provided))
}

fn subject_token_exchanger(
    SubjectTokenExchangerBinding { declaring, issuer }: &SubjectTokenExchangerBinding,
) -> FrameworkProvider {
    constructed(
        vec![
            FrameworkDependency::Provider(trusted_issuer_item_path(
                &issuer.trust.tag,
                TrustedIssuerItem::TrustedIssuer,
            )),
            FrameworkDependency::SingletonView(declaring.clone()),
        ],
        ConstructorOutcome::Infallible,
        FrameworkEnablement::Dependency,
        subject_token_exchanger_path(&issuer.trust.tag),
    )
}

fn aggregate(aggregate: ProviderAggregate) -> FrameworkDependency {
    FrameworkDependency::Constant(provider_aggregate_path(aggregate))
}

fn served_endpoint_providers(
    endpoint_routes: &DeclaredEndpointRoutes,
    userinfo: &UserinfoProvision,
) -> Vec<FrameworkProvider> {
    let secret_store = || FrameworkDependency::Provider(server_secret_store_canonical_path());
    let mut served = Vec::new();

    served.extend(
        endpoint_routes
            .serves(ProviderEndpoint::Discovery)
            .then(|| {
                served_endpoint(
                    vec![
                        aggregate(ProviderAggregate::ProviderSupport),
                        FrameworkDependency::Constant(provider_endpoints_path()),
                        FrameworkDependency::TokenIssuance,
                    ],
                    ConstructorOutcome::Fallible,
                    OidcProviderItem::ProviderMetadataHandler,
                )
            }),
    );
    served.extend(
        endpoint_routes
            .serves(ProviderEndpoint::Introspection)
            .then(|| {
                served_endpoint(
                    vec![item(OidcProviderItem::AcceptedClients), secret_store()],
                    ConstructorOutcome::Infallible,
                    OidcProviderItem::IntrospectionEndpoint,
                )
            }),
    );
    served.extend(
        endpoint_routes
            .serves(ProviderEndpoint::Revocation)
            .then(|| {
                served_endpoint(
                    vec![
                        item(OidcProviderItem::AcceptedClients),
                        secret_store(),
                        grants_database(),
                        aggregate(ProviderAggregate::AcceptedResources),
                    ],
                    ConstructorOutcome::Infallible,
                    OidcProviderItem::RevocationEndpoint,
                )
            }),
    );
    served.extend(endpoint_routes.serves(ProviderEndpoint::Token).then(|| {
        served_endpoint(
            vec![
                item(OidcProviderItem::AcceptedClients),
                item(OidcProviderItem::SubjectTokenExchangers),
                secret_store(),
                FrameworkDependency::TokenIssuance,
            ],
            ConstructorOutcome::Infallible,
            OidcProviderItem::TokenEndpoint,
        )
    }));

    if let UserinfoProvision::Served { claims } = userinfo {
        served.push(served_endpoint(
            vec![
                secret_store(),
                FrameworkDependency::TokenIssuance,
                FrameworkDependency::SingletonView(claims.clone()),
            ],
            ConstructorOutcome::Infallible,
            OidcProviderItem::UserinfoEndpoint,
        ));
    }

    served
}

pub(crate) fn oidc_provider_framework_providers(
    registered_clients: Vec<CanonicalPath>,
    exchangers: &[SubjectTokenExchangerBinding],
    ServedEndpoints {
        provider,
        routes,
        userinfo,
        ..
    }: &ServedEndpoints,
) -> Vec<FrameworkProvider> {
    let mut providers = vec![
        constructed(
            vec![
                FrameworkDependency::Providers(registered_clients),
                FrameworkDependency::TokenIssuance,
            ],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::AcceptedClients),
        ),
        constructed(
            vec![FrameworkDependency::Providers(
                exchangers
                    .iter()
                    .map(|binding| subject_token_exchanger_path(&binding.issuer.trust.tag))
                    .collect(),
            )],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::SubjectTokenExchangers),
        ),
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Constant(provider_endpoints_path())],
                is_async: false,
                method: "of_provider".to_string(),
                outcome: ConstructorOutcome::Fallible,
            },
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::Unmarked,
            provided: oidc_provider_item_path(OidcProviderItem::IssuerMetadata),
        },
    ];

    providers.extend(served_endpoint_providers(routes, userinfo));

    if let DerivedAuthorization::Served(served) = provider.authorization() {
        providers.extend(authorization_providers(served));
    }

    providers.extend(exchangers.iter().map(subject_token_exchanger));

    providers
}
