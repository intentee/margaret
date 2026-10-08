use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::derived_provider_endpoints::DerivedProviderEndpoints;
use crate::endpoint_routes::EndpointRoutes;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::provider_endpoint::ProviderEndpoint;

/// # Errors
///
/// Returns `OidcProviderCodegenError` when the discovery document is not served by exactly one
/// server at the discovery location of the issuer, when no route uses the consent endpoint, or
/// when an endpoint is not served at exactly one parameterless path of that server by routes of a
/// method it admits.
pub fn derive_provider_endpoints(
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
    issuer: &IssuerIdentifier,
    capable: &[ProviderEndpoint],
) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
    let routes = EndpointRoutes {
        bindings,
        locations,
    };
    let server = routes.provider_server()?;

    routes.consent(capable)?;
    routes.discovery_served(server, issuer)?;

    Ok(DerivedProviderEndpoints {
        authorization: routes.optional(ProviderEndpoint::Authorization, capable, server, issuer)?,
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
    use std::collections::BTreeMap;

    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_enablement::FrameworkEnablement;
    use margaret_container::framework_injection_role::FrameworkInjectionRole;
    use margaret_container::framework_provider::FrameworkProvider;
    use margaret_container::render_container::render_container;
    use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
    use margaret_http_codegen::declared_routes::DeclaredRoutes;
    use margaret_http_codegen::http_plan::HttpPlan;
    use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_tag_codegen_tests::collected_tags::collected_tags;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::derive_provider_endpoints;
    use crate::derived_endpoint::DerivedEndpoint;
    use crate::derived_provider_endpoints::DerivedProviderEndpoints;
    use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
    use crate::oidc_provider_item::OidcProviderItem;
    use crate::oidc_provider_item_path::oidc_provider_item_path;
    use crate::provider_endpoint::ProviderEndpoint;

    const ENDPOINTS: [ProviderEndpoint; 7] = [
        ProviderEndpoint::Authorization,
        ProviderEndpoint::Discovery,
        ProviderEndpoint::Introspection,
        ProviderEndpoint::Jwks,
        ProviderEndpoint::Revocation,
        ProviderEndpoint::Token,
        ProviderEndpoint::Userinfo,
    ];

    #[derive(Clone, Copy)]
    struct FixtureRoute {
        handler: &'static str,
        method: &'static str,
        name: &'static str,
        path: &'static str,
        server: &'static str,
    }

    fn route(
        name: &'static str,
        method: &'static str,
        path: &'static str,
        handler: &'static str,
    ) -> FixtureRoute {
        FixtureRoute {
            handler,
            method,
            name,
            path,
            server: "public",
        }
    }

    fn provider_routes() -> Vec<FixtureRoute> {
        vec![
            route(
                "GetAuthorize",
                "Get",
                "/authorize",
                "oidc_provider::AuthorizationEndpoint",
            ),
            route(
                "PostAuthorize",
                "Post",
                "/authorize",
                "oidc_provider::AuthorizationEndpoint",
            ),
            route(
                "PostConsent",
                "Post",
                "/consent",
                "oidc_provider::ConsentEndpoint",
            ),
            route(
                "GetDiscovery",
                "Get",
                "/.well-known/openid-configuration",
                "oidc_provider::ProviderMetadataHandler",
            ),
            route(
                "PostIntrospect",
                "Post",
                "/introspect",
                "oidc_provider::IntrospectionEndpoint",
            ),
            route("GetJwks", "Get", "/jwks.json", "jwks::PublicJwksHandler"),
            route(
                "PostRevoke",
                "Post",
                "/revoke",
                "oidc_provider::RevocationEndpoint",
            ),
            route(
                "PostToken",
                "Post",
                "/token",
                "oidc_provider::TokenEndpoint",
            ),
            route(
                "GetUserinfo",
                "Get",
                "/userinfo",
                "oidc_provider::UserinfoEndpoint",
            ),
        ]
    }

    fn source_of(routes: &[FixtureRoute]) -> String {
        routes
            .iter()
            .map(
                |FixtureRoute {
                     handler,
                     method,
                     name,
                     path,
                     server,
                 }| {
                    format!(
                        "#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::{method}, path = \"{path}\", server = \"{server}\")]\nstruct {name};\n\nimpl {name} {{\n    #[constructor]\n    fn create(handler: std::sync::Arc<crate::margaret::{handler}>) -> anyhow::Result<Self> {{}}\n\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {{}}\n}}\n\n"
                    )
                },
            )
            .collect::<Vec<String>>()
            .concat()
    }

    const EVERY_CAPABILITY: &[ProviderEndpoint] = &[
        ProviderEndpoint::Authorization,
        ProviderEndpoint::Introspection,
        ProviderEndpoint::Revocation,
        ProviderEndpoint::Userinfo,
    ];

    const NO_CAPABILITY: &[ProviderEndpoint] = &[];

    fn derived_for(
        routes: &[FixtureRoute],
        issuer: &str,
        capable: &[ProviderEndpoint],
    ) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
        let indexed = IndexedSource::new(&source_of(routes));
        let index = &indexed.index;
        let handlers: Vec<FrameworkProvider> = ENDPOINTS
            .into_iter()
            .map(ProviderEndpoint::handler_path)
            .chain([oidc_provider_item_path(OidcProviderItem::ConsentEndpoint)])
            .map(|provided| FrameworkProvider {
                construction: FrameworkConstruction::Unit,
                enablement: FrameworkEnablement::WhenReferenced,
                injection: FrameworkInjectionRole::Unmarked,
                provided,
            })
            .collect();
        let bindings = render_container(
            index,
            &scan(index).expect("the serve inputs are scanned"),
            &handlers,
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings;
        let tags = collected_tags(index);
        let registries =
            BindingRegistries::collect(index, ViewsAvailability::Unavailable, &tags, &bindings)
                .expect("the registries are collected");
        let middleware_plans = MiddlewarePlans::collect(index, &registries, &tags)
            .expect("the middleware plans are collected");
        let plan = HttpPlan::build(
            index,
            DeclaredRoutes::read(index).expect("the routes are declared"),
            false,
            &BTreeMap::new(),
            &middleware_plans,
            &registries,
        )
        .expect("the http plan builds");

        derive_provider_endpoints(
            &plan.route_locations(),
            &bindings,
            &issuer.parse().expect("the issuer is an https url"),
            capable,
        )
    }

    fn derived(
        routes: &[FixtureRoute],
    ) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
        derived_for(routes, "https://issuer.example", EVERY_CAPABILITY)
    }

    fn served(url: &str) -> DerivedEndpoint {
        DerivedEndpoint::Served(url.to_string())
    }

    fn required_routes() -> Vec<FixtureRoute> {
        provider_routes()
            .into_iter()
            .filter(|route| ["GetDiscovery", "GetJwks", "PostToken"].contains(&route.name))
            .collect()
    }

    fn rejection(routes: &[FixtureRoute]) -> String {
        derived(routes)
            .err()
            .expect("the provider routes are rejected")
            .to_string()
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

        assert_eq!(
            endpoints.authorization,
            served("https://issuer.example/authorize")
        );
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
    fn leaves_the_endpoints_no_admitted_client_needs_unserved() {
        let endpoints = derived_for(&required_routes(), "https://issuer.example", NO_CAPABILITY)
            .expect("the required endpoints are routed");

        assert_eq!(endpoints.authorization, DerivedEndpoint::Unserved);
        assert_eq!(endpoints.introspection, DerivedEndpoint::Unserved);
        assert_eq!(endpoints.revocation, DerivedEndpoint::Unserved);
        assert_eq!(endpoints.userinfo, DerivedEndpoint::Unserved);
    }

    #[test]
    fn rejects_a_route_of_an_endpoint_no_admitted_client_needs() {
        let routes: Vec<FixtureRoute> = provider_routes()
            .into_iter()
            .filter(|route| {
                ["GetDiscovery", "GetJwks", "PostToken", "PostIntrospect"].contains(&route.name)
            })
            .collect();

        assert_eq!(
            derived_for(&routes, "https://issuer.example", NO_CAPABILITY)
                .err()
                .expect("the introspection route is rejected")
                .to_string(),
            "the introspection endpoint is routed, but no admitted client introspects tokens"
        );
    }

    #[test]
    fn rejects_a_consent_route_of_a_provider_that_grants_no_authorization_codes() {
        let routes: Vec<FixtureRoute> = provider_routes()
            .into_iter()
            .filter(|route| {
                ["GetDiscovery", "GetJwks", "PostToken", "PostConsent"].contains(&route.name)
            })
            .collect();

        assert_eq!(
            derived_for(&routes, "https://issuer.example", NO_CAPABILITY)
                .err()
                .expect("the consent route is rejected")
                .to_string(),
            "a route serves the consent endpoint, but no admitted client grants authorization codes an end user could consent to"
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
        let routes: Vec<FixtureRoute> = provider_routes()
            .into_iter()
            .filter(|route| route.name != "GetDiscovery")
            .collect();

        assert_eq!(rejection(&routes), "no route serves the discovery document");
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
                "oidc_provider::ProviderMetadataHandler",
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
                    ..route("", "Get", "/userinfo", "oidc_provider::UserinfoEndpoint")
                },
            )),
            "no route of the server 'public' serves the userinfo endpoint"
        );
    }

    #[test]
    fn rejects_an_endpoint_served_at_several_paths() {
        let mut routes = provider_routes();

        routes.push(route(
            "GetKeys",
            "Get",
            "/keys.json",
            "jwks::PublicJwksHandler",
        ));

        assert_eq!(
            rejection(&routes),
            r#"the jwks document is served at several paths of the server 'public': ["/jwks.json", "/keys.json"]"#
        );
    }

    #[test]
    fn rejects_an_endpoint_route_of_a_method_it_does_not_admit() {
        assert_eq!(
            rejection(&replaced(
                "PostToken",
                route("", "Get", "/token", "oidc_provider::TokenEndpoint"),
            )),
            "the token endpoint is served by a Get route at '/token', which it does not admit"
        );
    }

    #[test]
    fn rejects_a_parameterized_endpoint_route() {
        assert_eq!(
            rejection(&replaced(
                "PostRevoke",
                route(
                    "",
                    "Post",
                    "/revoke/{tenant}",
                    "oidc_provider::RevocationEndpoint"
                ),
            )),
            "the revocation endpoint is served at '/revoke/{tenant}', which has route parameters"
        );
    }

    #[test]
    fn rejects_an_authorization_route_of_a_method_other_than_get_or_post() {
        assert_eq!(
            rejection(&replaced(
                "PostAuthorize",
                route(
                    "",
                    "Put",
                    "/authorize",
                    "oidc_provider::AuthorizationEndpoint"
                ),
            )),
            "the authorization endpoint is served by a Put route at '/authorize', which it does not admit"
        );
    }

    #[test]
    fn rejects_a_discovery_document_served_by_a_post_route() {
        assert_eq!(
            rejection(&replaced(
                "GetDiscovery",
                route(
                    "",
                    "Post",
                    "/.well-known/openid-configuration",
                    "oidc_provider::ProviderMetadataHandler"
                ),
            )),
            "the discovery document is served by a Post route at '/.well-known/openid-configuration', which it does not admit"
        );
    }

    #[test]
    fn rejects_a_provider_without_a_consent_route() {
        let routes: Vec<FixtureRoute> = provider_routes()
            .into_iter()
            .filter(|route| route.name != "PostConsent")
            .collect();

        assert_eq!(
            rejection(&routes),
            "no route uses the consent endpoint, so the authorization endpoint cannot ask an end user for consent"
        );
    }

    #[test]
    fn rejects_a_discovery_document_outside_its_well_known_path() {
        assert_eq!(
            rejection(&replaced(
                "GetDiscovery",
                route(
                    "",
                    "Get",
                    "/openid-configuration",
                    "oidc_provider::ProviderMetadataHandler"
                ),
            )),
            "the discovery document is served at '/openid-configuration', but its issuer publishes it at '/.well-known/openid-configuration'"
        );
    }

    #[test]
    fn rejects_a_provider_without_an_introspection_route() {
        let routes: Vec<FixtureRoute> = provider_routes()
            .into_iter()
            .filter(|route| route.name != "PostIntrospect")
            .collect();

        assert_eq!(
            rejection(&routes),
            "no route of the server 'public' serves the introspection endpoint"
        );
    }

    #[test]
    fn describes_every_provider_endpoint() {
        assert_eq!(
            ENDPOINTS.map(|endpoint| endpoint.to_string()),
            [
                "authorization endpoint",
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
