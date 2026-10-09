use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_session_endpoints_codegen::declared_session_endpoint_route::DeclaredSessionEndpointRoute;
use margaret_session_endpoints_codegen::declared_session_endpoints::DeclaredSessionEndpoints;
use margaret_session_endpoints_codegen::served_session_endpoint::ServedSessionEndpoint;
use margaret_sessions_codegen::consumed_sessions_declaration::ConsumedSessionsDeclaration;
use margaret_sessions_codegen::declared_session_cookies::DeclaredSessionCookies;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_sessions_codegen::issued_sessions_declaration::IssuedSessionsDeclaration;
use margaret_sessions_codegen::session_audience_path::session_audience_path;
use margaret_sessions_codegen::sessions_item::SessionsItem;
use margaret_sessions_codegen::sessions_item_path::sessions_item_path;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;
use crate::sessions_tables_canonical_path::sessions_tables_canonical_path;

fn sessions_type_path(module: &str, type_name: &str) -> CanonicalPath {
    CanonicalPath::new(
        ["margaret", "framework", "sessions", module, type_name]
            .iter()
            .map(ToString::to_string)
            .collect(),
    )
}

fn declared(
    dependencies: Vec<FrameworkDependency>,
    method: &str,
    injection: FrameworkInjectionRole,
    item: SessionsItem,
) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: method.to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::Declared,
        injection,
        provided: sessions_item_path(item),
    }
}

fn sessions_providers(sessions: &DeclaredSessions) -> Vec<FrameworkProvider> {
    match sessions {
        DeclaredSessions::Absent => Vec::new(),
        DeclaredSessions::Consumed(ConsumedSessionsDeclaration {
            cookie_domain_from,
            issuer,
            refresh_url_from,
            ..
        }) => vec![declared(
            vec![
                FrameworkDependency::Provider(trusted_issuer_item_path(
                    issuer,
                    TrustedIssuerItem::TrustedIssuer,
                )),
                FrameworkDependency::EnvironmentVariable {
                    name: cookie_domain_from.clone(),
                    value_type: sessions_type_path("cookie_domain", "CookieDomain"),
                },
                FrameworkDependency::EnvironmentVariable {
                    name: refresh_url_from.clone(),
                    value_type: sessions_type_path("session_refresh_url", "SessionRefreshUrl"),
                },
                FrameworkDependency::SpiffeHttpClient,
            ],
            "create",
            FrameworkInjectionRole::Unmarked,
            SessionsItem::ConsumedSessions,
        )],
        DeclaredSessions::Issued(IssuedSessionsDeclaration { cookies, .. }) => {
            let mut dependencies = vec![
                FrameworkDependency::Database {
                    framework_tables: vec![sessions_tables_canonical_path()],
                },
                FrameworkDependency::Provider(server_secret_store_canonical_path()),
                FrameworkDependency::Constant(session_audience_path()),
            ];
            let method = match cookies {
                DeclaredSessionCookies::HostOnly => "host_only",
                DeclaredSessionCookies::SharedWithDomain { domain_from } => {
                    dependencies.push(FrameworkDependency::EnvironmentVariable {
                        name: domain_from.clone(),
                        value_type: sessions_type_path("cookie_domain", "CookieDomain"),
                    });

                    "shared_with_domain"
                }
            };

            vec![declared(
                dependencies,
                method,
                FrameworkInjectionRole::Unmarked,
                SessionsItem::IssuedSessions,
            )]
        }
    }
}

fn endpoint_provider(
    DeclaredSessionEndpointRoute { endpoint, .. }: &DeclaredSessionEndpointRoute,
) -> FrameworkProvider {
    let issued = FrameworkDependency::Provider(sessions_item_path(SessionsItem::IssuedSessions));

    match endpoint {
        ServedSessionEndpoint::Refresh => declared(
            vec![issued],
            "create",
            FrameworkInjectionRole::FrameworkState,
            SessionsItem::SessionRefreshEndpoint,
        ),
        ServedSessionEndpoint::SignOut { landing } => declared(
            vec![issued, FrameworkDependency::RouteUrl(landing.clone())],
            "create",
            FrameworkInjectionRole::FrameworkState,
            SessionsItem::SessionSignOutEndpoint,
        ),
    }
}

pub(crate) fn sessions_framework_providers(
    sessions: &DeclaredSessions,
    endpoints: &DeclaredSessionEndpoints,
) -> Vec<FrameworkProvider> {
    sessions_providers(sessions)
        .into_iter()
        .chain(endpoints.routes.iter().map(endpoint_provider))
        .collect()
}
