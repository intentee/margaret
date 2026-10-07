use std::collections::BTreeSet;

use url::Url;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;

use crate::derived_provider_endpoints::DerivedProviderEndpoints;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::oidc_provider_item::OidcProviderItem;
use crate::oidc_provider_item_path::oidc_provider_item_path;
use crate::provider_endpoint::ProviderEndpoint;

fn located_at_issuer(issuer: &IssuerIdentifier, path: &str) -> Url {
    let mut located = issuer.url().clone();

    located.set_path(path);

    located
}

struct EndpointRoutes<'plan> {
    bindings: &'plan ContainerBindings,
    locations: &'plan [RouteLocation<'plan>],
}

impl<'plan> EndpointRoutes<'plan> {
    fn path(
        &self,
        endpoint: ProviderEndpoint,
        server: &str,
    ) -> Result<String, OidcProviderCodegenError> {
        let mut paths = BTreeSet::new();

        for location in self
            .serving(endpoint)
            .into_iter()
            .filter(|location| location.server == server)
        {
            let path = location.path.pattern();

            if !endpoint.admits(location.method) {
                return Err(OidcProviderCodegenError::EndpointRouteMethod {
                    endpoint,
                    method: location.method,
                    path: path.to_string(),
                });
            }

            if location.path.parameters().next().is_some() {
                return Err(OidcProviderCodegenError::ParameterizedEndpointRoute {
                    endpoint,
                    path: path.to_string(),
                });
            }

            paths.insert(path.to_string());
        }

        match paths.first() {
            Some(path) if paths.len() == 1 => Ok(path.clone()),
            Some(_) => Err(OidcProviderCodegenError::AmbiguousEndpointRoute {
                endpoint,
                paths: paths.into_iter().collect(),
                server: server.to_string(),
            }),
            None => Err(OidcProviderCodegenError::MissingEndpointRoute {
                endpoint,
                server: server.to_string(),
            }),
        }
    }

    fn consent_served(&self) -> Result<(), OidcProviderCodegenError> {
        if self.serves(&oidc_provider_item_path(OidcProviderItem::ConsentEndpoint)) {
            Ok(())
        } else {
            Err(OidcProviderCodegenError::MissingConsentRoute)
        }
    }

    fn discovery_served(
        &self,
        server: &str,
        issuer: &IssuerIdentifier,
    ) -> Result<(), OidcProviderCodegenError> {
        let path = self.path(ProviderEndpoint::Discovery, server)?;
        let expected = oidc_discovery_url(issuer);

        if located_at_issuer(issuer, &path) == expected {
            Ok(())
        } else {
            Err(OidcProviderCodegenError::DiscoveryPathMismatch {
                expected: expected.path().to_string(),
                path,
            })
        }
    }

    fn url(
        &self,
        endpoint: ProviderEndpoint,
        server: &str,
        issuer: &IssuerIdentifier,
    ) -> Result<String, OidcProviderCodegenError> {
        self.path(endpoint, server)
            .map(|path| located_at_issuer(issuer, &path).to_string())
    }

    fn provider_server(&self) -> Result<&'plan str, OidcProviderCodegenError> {
        let servers = self
            .serving(ProviderEndpoint::Discovery)
            .into_iter()
            .map(|location| location.server)
            .collect::<BTreeSet<&str>>();
        match servers.first() {
            Some(server) if servers.len() == 1 => Ok(server),
            Some(_) => Err(OidcProviderCodegenError::DiscoveryServedBySeveralServers {
                servers: servers.iter().map(ToString::to_string).collect(),
            }),
            None => Err(OidcProviderCodegenError::MissingDiscoveryRoute),
        }
    }

    fn serves(&self, handler: &CanonicalPath) -> bool {
        self.locations.iter().any(|location| {
            self.bindings
                .depends_directly_on(location.responder_path, handler)
        })
    }

    fn serving(&self, endpoint: ProviderEndpoint) -> Vec<&'plan RouteLocation<'plan>> {
        let handler = endpoint.handler_path();

        self.locations
            .iter()
            .filter(|location| {
                self.bindings
                    .depends_directly_on(location.responder_path, &handler)
            })
            .collect()
    }
}

/// # Errors
///
/// Returns `OidcProviderCodegenError` when the discovery document is not served by exactly one
/// server at the discovery location of the issuer, when no route uses the consent endpoint, or
/// when an endpoint is not served at exactly one parameterless path of that server by routes of a
/// method it admits.
pub fn derive_provider_endpoints(
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
    TokenIssuanceDeclaration { issuer, .. }: &TokenIssuanceDeclaration,
) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
    let routes = EndpointRoutes {
        bindings,
        locations,
    };
    let server = routes.provider_server()?;

    routes.consent_served()?;
    routes.discovery_served(server, issuer)?;

    Ok(DerivedProviderEndpoints {
        authorization: routes.url(ProviderEndpoint::Authorization, server, issuer)?,
        introspection: routes.url(ProviderEndpoint::Introspection, server, issuer)?,
        issuer_origin: issuer.url().origin().ascii_serialization(),
        jwks: routes.url(ProviderEndpoint::Jwks, server, issuer)?,
        revocation: routes.url(ProviderEndpoint::Revocation, server, issuer)?,
        server: server.to_string(),
        token: routes.url(ProviderEndpoint::Token, server, issuer)?,
        userinfo: routes.url(ProviderEndpoint::Userinfo, server, issuer)?,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_enablement::FrameworkEnablement;
    use margaret_container::framework_injection_role::FrameworkInjectionRole;
    use margaret_container::framework_provider::FrameworkProvider;
    use margaret_container::render_container::render_container;
    use margaret_http_codegen::http_plan::HttpPlan;
    use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
    use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_tag_codegen::tag_pool::TagPool;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
    use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;
    use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;

    use super::derive_provider_endpoints;
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
                "get",
                "/authorize",
                "oidc_provider::AuthorizationEndpoint",
            ),
            route(
                "PostAuthorize",
                "post",
                "/authorize",
                "oidc_provider::AuthorizationEndpoint",
            ),
            route(
                "PostConsent",
                "post",
                "/consent",
                "oidc_provider::ConsentEndpoint",
            ),
            route(
                "GetDiscovery",
                "get",
                "/.well-known/openid-configuration",
                "oidc_provider::ProviderMetadataHandler",
            ),
            route(
                "PostIntrospect",
                "post",
                "/introspect",
                "oidc_provider::IntrospectionEndpoint",
            ),
            route("GetJwks", "get", "/jwks.json", "jwks::PublicJwksHandler"),
            route(
                "PostRevoke",
                "post",
                "/revoke",
                "oidc_provider::RevocationEndpoint",
            ),
            route(
                "PostToken",
                "post",
                "/token",
                "oidc_provider::TokenEndpoint",
            ),
            route(
                "GetUserinfo",
                "get",
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
                        "#[singleton]\n#[responds_to_http(method = \"{method}\", path = \"{path}\", server = \"{server}\")]\nstruct {name};\n\nimpl {name} {{\n    #[constructor]\n    fn create(handler: std::sync::Arc<crate::margaret::{handler}>) -> anyhow::Result<Self> {{}}\n\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {{}}\n}}\n\n"
                    )
                },
            )
            .collect::<Vec<String>>()
            .concat()
    }

    fn issuance(issuer: &str) -> TokenIssuanceDeclaration {
        TokenIssuanceDeclaration {
            anchor: CanonicalPath::new(vec!["crate".to_string(), "Issuer".to_string()]),
            audience: "session".parse().expect("the audience is not empty"),
            issuer: issuer.parse().expect("the issuer is an https url"),
        }
    }

    fn derived_for(
        routes: &[FixtureRoute],
        issuer: &str,
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
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings;
        let tags = TagPool::collect(
            index,
            &DeclaredTrusts::read(index).expect("the trusts are read"),
            &DeclaredOAuthClients::read(index).expect("the oauth clients are read"),
        )
        .expect("the tags are collected");
        let registries =
            BindingRegistries::collect(index, ViewsAvailability::Unavailable, &tags, &bindings)
                .expect("the registries are collected");
        let middleware_plans = MiddlewarePlans::collect(index, &registries, &tags)
            .expect("the middleware plans are collected");
        let plan = HttpPlan::build(
            index,
            false,
            &BTreeMap::new(),
            &middleware_plans,
            &bindings,
            &registries,
        )
        .expect("the http plan builds");

        derive_provider_endpoints(&plan.route_locations(), &bindings, &issuance(issuer))
    }

    fn derived(
        routes: &[FixtureRoute],
    ) -> Result<DerivedProviderEndpoints, OidcProviderCodegenError> {
        derived_for(routes, "https://issuer.example")
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

        assert_eq!(endpoints.authorization, "https://issuer.example/authorize");
        assert_eq!(endpoints.introspection, "https://issuer.example/introspect");
        assert_eq!(endpoints.issuer_origin, "https://issuer.example");
        assert_eq!(endpoints.jwks, "https://issuer.example/jwks.json");
        assert_eq!(endpoints.revocation, "https://issuer.example/revoke");
        assert_eq!(endpoints.server, "public");
        assert_eq!(endpoints.token, "https://issuer.example/token");
        assert_eq!(endpoints.userinfo, "https://issuer.example/userinfo");
    }

    #[test]
    fn rejects_a_root_discovery_route_of_an_issuer_with_a_path() {
        assert_eq!(
            derived_for(&provider_routes(), "https://issuer.example/tenant")
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
                "get",
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
                    ..route("", "get", "/userinfo", "oidc_provider::UserinfoEndpoint")
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
            "get",
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
                route("", "get", "/token", "oidc_provider::TokenEndpoint"),
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
                    "post",
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
                    "put",
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
                    "post",
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
                    "get",
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
