use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;

use crate::declared_endpoint_routes::DeclaredEndpointRoutes;
use crate::derived_provider_endpoints::DerivedProviderEndpoints;
use crate::endpoint_routes::EndpointRoutes;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::provider_endpoint::ProviderEndpoint;

/// # Errors
///
/// Returns `OidcProviderCodegenError` when the discovery document is not served by exactly one
/// server at the discovery location of the issuer, when an endpoint is not served at exactly one
/// parameterless path of that server, when an endpoint is served that no admitted client uses,
/// or when the authorization endpoint is served without a consent page of that server or without
/// sessions that authenticate its end users.
pub fn derive_provider_endpoints(
    marked: &DeclaredEndpointRoutes,
    issuer: &IssuerIdentifier,
    capable: &[ProviderEndpoint],
    sessions: &DeclaredSessions,
) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
    let routes = EndpointRoutes { marked };
    let server = routes.provider_server()?;

    routes.discovery_served(server, issuer)?;

    Ok(DerivedProviderEndpoints {
        authorization: routes.authorization(capable, server, issuer, sessions)?,
        introspection: routes.optional(ProviderEndpoint::Introspection, capable, server, issuer)?,
        issuer_origin: issuer.url().origin().ascii_serialization(),
        jwks: routes.url(ProviderEndpoint::Jwks, server, issuer)?,
        revocation: routes.optional(ProviderEndpoint::Revocation, capable, server, issuer)?,
        server: server.to_string(),
        token: routes.url(ProviderEndpoint::Token, server, issuer)?,
        userinfo: routes.optional(ProviderEndpoint::Userinfo, capable, server, issuer)?,
    })
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_http_codegen::declared_routes::DeclaredRoutes;
    use margaret_serve_input_codegen::route_url_input::RouteUrlInput;
    use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::derive_provider_endpoints;
    use crate::consent_page::ConsentPage;
    use crate::declared_endpoint_routes::DeclaredEndpointRoutes;
    use crate::derived_authorization::DerivedAuthorization;
    use crate::derived_endpoint::DerivedEndpoint;
    use crate::derived_provider_endpoints::DerivedProviderEndpoints;
    use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
    use crate::provider_endpoint::ProviderEndpoint;
    use crate::served_authorization::ServedAuthorization;

    const ISSUANCE: &str = "use margaret::framework::sessions::session_cookies::SessionCookies;\n\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n\n#[renders_view(name = \"consent_view\")]\n#[singleton]\npub struct ConsentView;\n\n";

    const SESSIONS: &str = "#[issues_sessions(issuer = provider, audience = \"browser\", cookies = SessionCookies::HostOnly)]\npub struct BrowserSessions;\n\n";

    const ENDPOINTS: [ProviderEndpoint; 8] = [
        ProviderEndpoint::Authorization,
        ProviderEndpoint::Consent,
        ProviderEndpoint::Discovery,
        ProviderEndpoint::Introspection,
        ProviderEndpoint::Jwks,
        ProviderEndpoint::Revocation,
        ProviderEndpoint::Token,
        ProviderEndpoint::Userinfo,
    ];

    const EVERY_CAPABILITY: &[ProviderEndpoint] = &[
        ProviderEndpoint::Authorization,
        ProviderEndpoint::Introspection,
        ProviderEndpoint::Revocation,
        ProviderEndpoint::Userinfo,
    ];

    const NO_CAPABILITY: &[ProviderEndpoint] = &[];

    #[derive(Clone, Copy)]
    struct FixtureRoute {
        endpoint: &'static str,
        method: &'static str,
        name: &'static str,
        path: &'static str,
        server: &'static str,
    }

    fn route(
        name: &'static str,
        method: &'static str,
        path: &'static str,
        endpoint: &'static str,
    ) -> FixtureRoute {
        FixtureRoute {
            endpoint,
            method,
            name,
            path,
            server: "public",
        }
    }

    fn provider_routes() -> Vec<FixtureRoute> {
        vec![
            route("GetAuthorize", "Get", "/authorize", "Authorization"),
            route("PostAuthorize", "Post", "/authorize", "Authorization"),
            route(
                "PostConsent",
                "Post",
                "/consent",
                "Consent(view = ConsentView)",
            ),
            route(
                "GetDiscovery",
                "Get",
                "/.well-known/openid-configuration",
                "Discovery",
            ),
            route("PostIntrospect", "Post", "/introspect", "Introspection"),
            route("GetJwks", "Get", "/jwks.json", "Jwks"),
            route("PostRevoke", "Post", "/revoke", "Revocation"),
            route("PostToken", "Post", "/token", "Token"),
            route("GetUserinfo", "Get", "/userinfo", "Userinfo"),
        ]
    }

    fn source_of(routes: &[FixtureRoute]) -> String {
        routes
            .iter()
            .map(
                |FixtureRoute {
                     endpoint,
                     method,
                     name,
                     path,
                     server,
                 }| {
                    format!(
                        "#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::{method}, path = \"{path}\", server = \"{server}\")]\n#[serves_oidc_endpoint(margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint::{endpoint})]\nstruct {name};\n\n"
                    )
                },
            )
            .collect::<Vec<String>>()
            .concat()
    }

    fn derived_with(
        sessions: &str,
        routes: &[FixtureRoute],
        issuer: &str,
        capable: &[ProviderEndpoint],
    ) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
        let indexed = IndexedSource::new(&format!("{ISSUANCE}{sessions}{}", source_of(routes)));
        let index = &indexed.index;
        let issuance = DeclaredTokenIssuance::read(index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(index, &issuance)
            .expect("the resource issuances are read");
        let declared = DeclaredRoutes::read(index).expect("the routes are declared");

        derive_provider_endpoints(
            &DeclaredEndpointRoutes::read(index, &declared).expect("the endpoint routes are read"),
            &issuer.parse().expect("the issuer is an https url"),
            capable,
            &DeclaredSessions::read(index, &issuance, &resources).expect("the sessions are read"),
        )
    }

    fn derived_for(
        routes: &[FixtureRoute],
        issuer: &str,
        capable: &[ProviderEndpoint],
    ) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
        derived_with(SESSIONS, routes, issuer, capable)
    }

    fn derived(
        routes: &[FixtureRoute],
    ) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
        derived_for(routes, "https://issuer.example", EVERY_CAPABILITY)
    }

    fn served(url: &str) -> DerivedEndpoint {
        DerivedEndpoint::Served(url.to_string())
    }

    fn rejection(routes: &[FixtureRoute]) -> String {
        derived(routes)
            .err()
            .expect("the provider routes are rejected")
            .to_string()
    }

    fn named(names: &[&str]) -> Vec<FixtureRoute> {
        provider_routes()
            .into_iter()
            .filter(|route| names.contains(&route.name))
            .collect()
    }

    fn without(name: &str) -> Vec<FixtureRoute> {
        provider_routes()
            .into_iter()
            .filter(|route| route.name != name)
            .collect()
    }

    fn replaced(name: &'static str, replacement: FixtureRoute) -> Vec<FixtureRoute> {
        provider_routes()
            .into_iter()
            .map(|route| {
                if route.name == name {
                    FixtureRoute {
                        name,
                        ..replacement
                    }
                } else {
                    route
                }
            })
            .collect()
    }

    #[test]
    fn locates_every_endpoint_at_the_issuer() {
        let endpoints = derived(&provider_routes()).expect("every endpoint is routed");

        assert!(matches!(
            endpoints.authorization,
            DerivedAuthorization::Served(ServedAuthorization {
                consent: ConsentPage {
                    decision: RouteUrlInput { path, server },
                    view,
                },
                url,
            }) if path == "/consent"
                && server == "public"
                && view.to_string() == "crate::ConsentView"
                && url == "https://issuer.example/authorize"
        ));
        assert_eq!(
            endpoints.introspection,
            served("https://issuer.example/introspect")
        );
        assert_eq!(endpoints.issuer_origin, "https://issuer.example");
        assert_eq!(endpoints.jwks, "https://issuer.example/jwks.json");
        assert_eq!(
            endpoints.revocation,
            served("https://issuer.example/revoke")
        );
        assert_eq!(endpoints.server, "public");
        assert_eq!(endpoints.token, "https://issuer.example/token");
        assert_eq!(
            endpoints.userinfo,
            served("https://issuer.example/userinfo")
        );
    }

    #[test]
    fn locates_an_endpoint_route_with_escaped_braces_at_its_literal_path() {
        let endpoints = derived(&replaced(
            "PostToken",
            route("", "Post", "/oauth/{{token}}", "Token"),
        ))
        .expect("every endpoint is routed");

        assert_eq!(endpoints.token, "https://issuer.example/oauth/%7Btoken%7D");
    }

    #[test]
    fn leaves_the_endpoints_no_admitted_client_needs_unserved() {
        let endpoints = derived_for(
            &named(&["GetDiscovery", "GetJwks", "PostToken"]),
            "https://issuer.example",
            NO_CAPABILITY,
        )
        .expect("the required endpoints are routed");

        assert_eq!(endpoints.authorization, DerivedAuthorization::Unserved);
        assert_eq!(endpoints.introspection, DerivedEndpoint::Unserved);
        assert_eq!(endpoints.revocation, DerivedEndpoint::Unserved);
        assert_eq!(endpoints.userinfo, DerivedEndpoint::Unserved);
    }

    #[test]
    fn rejects_a_route_of_an_endpoint_no_admitted_client_needs() {
        assert_eq!(
            derived_for(
                &named(&["GetDiscovery", "GetJwks", "PostToken", "PostIntrospect"]),
                "https://issuer.example",
                NO_CAPABILITY
            )
            .err()
            .expect("the introspection route is rejected")
            .to_string(),
            "the introspection endpoint is routed, but no admitted client introspects tokens"
        );
    }

    #[test]
    fn rejects_an_authorization_route_of_a_provider_that_grants_no_authorization_codes() {
        assert_eq!(
            derived_for(
                &named(&["GetDiscovery", "GetJwks", "PostToken", "GetAuthorize"]),
                "https://issuer.example",
                NO_CAPABILITY
            )
            .err()
            .expect("the authorization route is rejected")
            .to_string(),
            "the authorization endpoint is routed, but no admitted client grants authorization codes"
        );
    }

    #[test]
    fn rejects_a_consent_route_of_a_provider_that_grants_no_authorization_codes() {
        assert_eq!(
            derived_for(
                &named(&["GetDiscovery", "GetJwks", "PostToken", "PostConsent"]),
                "https://issuer.example",
                NO_CAPABILITY
            )
            .err()
            .expect("the consent route is rejected")
            .to_string(),
            "the consent endpoint is routed, but no admitted client grants authorization codes"
        );
    }

    #[test]
    fn rejects_a_root_discovery_route_of_an_issuer_with_a_path() {
        assert_eq!(
            derived_for(
                &provider_routes(),
                "https://issuer.example/tenant",
                EVERY_CAPABILITY
            )
            .err()
            .expect("the discovery route is rejected")
            .to_string(),
            "the discovery document is served at '/.well-known/openid-configuration', but its issuer publishes it at '/tenant/.well-known/openid-configuration'"
        );
    }

    #[test]
    fn rejects_a_provider_without_a_discovery_route() {
        assert_eq!(
            rejection(&without("GetDiscovery")),
            "no route serves the discovery document"
        );
    }

    #[test]
    fn rejects_a_discovery_document_served_by_several_servers() {
        let mut routes = provider_routes();

        routes.push(FixtureRoute {
            server: "internal",
            ..route(
                "GetInternalDiscovery",
                "Get",
                "/.well-known/openid-configuration",
                "Discovery",
            )
        });

        assert_eq!(
            rejection(&routes),
            r#"the discovery document is served by several servers: ["internal", "public"]"#
        );
    }

    #[test]
    fn rejects_an_endpoint_without_a_route_of_the_provider_server() {
        assert_eq!(
            rejection(&replaced(
                "GetUserinfo",
                FixtureRoute {
                    server: "internal",
                    ..route("", "Get", "/userinfo", "Userinfo")
                },
            )),
            "no route of the server 'public' serves the userinfo endpoint"
        );
    }

    #[test]
    fn rejects_an_endpoint_served_at_several_paths() {
        let mut routes = provider_routes();

        routes.push(route("GetKeys", "Get", "/keys.json", "Jwks"));

        assert_eq!(
            rejection(&routes),
            r#"the jwks document is served at several paths of the server 'public': ["/jwks.json", "/keys.json"]"#
        );
    }

    #[test]
    fn rejects_a_discovery_document_served_at_several_paths() {
        let mut routes = provider_routes();

        routes.push(route(
            "GetOtherDiscovery",
            "Get",
            "/.well-known/other-configuration",
            "Discovery",
        ));

        assert_eq!(
            rejection(&routes),
            r#"the discovery document is served at several paths of the server 'public': ["/.well-known/openid-configuration", "/.well-known/other-configuration"]"#
        );
    }

    #[test]
    fn rejects_a_provider_without_a_token_route() {
        assert_eq!(
            rejection(&without("PostToken")),
            "no route of the server 'public' serves the token endpoint"
        );
    }

    #[test]
    fn rejects_a_parameterized_endpoint_route() {
        assert_eq!(
            rejection(&replaced(
                "PostRevoke",
                route("", "Post", "/revoke/{tenant}", "Revocation"),
            )),
            "the revocation endpoint is served at '/revoke/{tenant}', which has route parameters"
        );
    }

    #[test]
    fn rejects_a_provider_without_a_consent_route() {
        assert_eq!(
            rejection(&without("PostConsent")),
            "no route of the server 'public' serves the consent endpoint"
        );
    }

    #[test]
    fn rejects_a_consent_route_of_another_server() {
        assert_eq!(
            rejection(&replaced(
                "PostConsent",
                FixtureRoute {
                    server: "internal",
                    ..route("", "Post", "/consent", "Consent(view = ConsentView)")
                },
            )),
            "the consent endpoint 'crate::PostConsent' is served by the server 'internal', but the authorization endpoint is served by 'public'"
        );
    }

    #[test]
    fn rejects_a_parameterized_consent_route() {
        assert_eq!(
            rejection(&replaced(
                "PostConsent",
                route("", "Post", "/consent/{id}", "Consent(view = ConsentView)"),
            )),
            "the consent endpoint is served at '/consent/{id}', which has route parameters"
        );
    }

    #[test]
    fn rejects_an_authorization_endpoint_of_a_crate_without_issued_sessions() {
        assert_eq!(
            derived_with(
                "",
                &provider_routes(),
                "https://issuer.example",
                EVERY_CAPABILITY
            )
            .err()
            .expect("the authorization endpoint is rejected")
            .to_string(),
            "the authorization endpoint is routed, but the crate declares no #[issues_sessions] to authenticate its end users"
        );
    }

    #[test]
    fn rejects_a_discovery_document_outside_its_well_known_path() {
        assert_eq!(
            rejection(&replaced(
                "GetDiscovery",
                route("", "Get", "/openid-configuration", "Discovery"),
            )),
            "the discovery document is served at '/openid-configuration', but its issuer publishes it at '/.well-known/openid-configuration'"
        );
    }

    #[test]
    fn rejects_a_provider_without_an_introspection_route() {
        assert_eq!(
            rejection(&without("PostIntrospect")),
            "no route of the server 'public' serves the introspection endpoint"
        );
    }

    #[test]
    fn describes_every_provider_endpoint() {
        assert_eq!(
            ENDPOINTS.map(|endpoint| endpoint.to_string()),
            [
                "authorization endpoint",
                "consent endpoint",
                "discovery document",
                "introspection endpoint",
                "jwks document",
                "revocation endpoint",
                "token endpoint",
                "userinfo endpoint",
            ]
            .map(ToString::to_string)
        );
    }
}
