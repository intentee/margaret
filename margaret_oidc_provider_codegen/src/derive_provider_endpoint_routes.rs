use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_oidc_discovery::oidc_discovery_path::OIDC_DISCOVERY_PATH;

use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::oidc_provider_item::OidcProviderItem;
use crate::oidc_provider_item_path::oidc_provider_item_path;
use crate::provider_endpoint::ProviderEndpoint;
use crate::provider_endpoint_routes::ProviderEndpointRoutes;

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

    fn discovery_path(&self, server: &str) -> Result<String, OidcProviderCodegenError> {
        let path = self.path(ProviderEndpoint::Discovery, server)?;

        if path.ends_with(OIDC_DISCOVERY_PATH) {
            Ok(path)
        } else {
            Err(OidcProviderCodegenError::DiscoveryOutsideWellKnownPath { path })
        }
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
/// server at its well-known location, when no route uses the consent endpoint, or when an
/// endpoint is not served at exactly one parameterless path of that server by routes of a method
/// it admits.
pub fn derive_provider_endpoint_routes(
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
) -> Result<ProviderEndpointRoutes, OidcProviderCodegenError> {
    let routes = EndpointRoutes {
        bindings,
        locations,
    };
    let server = routes.provider_server()?;

    routes.consent_served()?;

    Ok(ProviderEndpointRoutes {
        authorization: routes.path(ProviderEndpoint::Authorization, server)?,
        discovery: routes.discovery_path(server)?,
        introspection: routes.path(ProviderEndpoint::Introspection, server)?,
        jwks: routes.path(ProviderEndpoint::Jwks, server)?,
        revocation: routes.path(ProviderEndpoint::Revocation, server)?,
        server: server.to_string(),
        token: routes.path(ProviderEndpoint::Token, server)?,
        userinfo: routes.path(ProviderEndpoint::Userinfo, server)?,
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
    use margaret_http_codegen::http_plan::HttpPlan;
    use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_tag_codegen::tag_pool::TagPool;

    use super::derive_provider_endpoint_routes;
    use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
    use crate::oidc_provider_item::OidcProviderItem;
    use crate::oidc_provider_item_path::oidc_provider_item_path;
    use crate::provider_endpoint::ProviderEndpoint;
    use crate::provider_endpoint_routes::ProviderEndpointRoutes;

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

    fn derived(
        routes: &[FixtureRoute],
    ) -> Result<ProviderEndpointRoutes, OidcProviderCodegenError> {
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
        )
        .expect("the container renders")
        .bindings;
        let tags = TagPool::collect(index).expect("the tags are collected");
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

        derive_provider_endpoint_routes(&plan.route_locations(), &bindings)
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
    fn derives_the_path_of_every_endpoint() {
        let routes = derived(&provider_routes()).expect("every endpoint is routed");

        assert_eq!(routes.authorization, "/authorize");
        assert_eq!(routes.discovery, "/.well-known/openid-configuration");
        assert_eq!(routes.introspection, "/introspect");
        assert_eq!(routes.jwks, "/jwks.json");
        assert_eq!(routes.revocation, "/revoke");
        assert_eq!(routes.server, "public");
        assert_eq!(routes.token, "/token");
        assert_eq!(routes.userinfo, "/userinfo");
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
            "the discovery document is served at '/openid-configuration', outside the '/.well-known/openid-configuration' location of its issuer"
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
