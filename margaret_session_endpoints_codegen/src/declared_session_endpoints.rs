use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_route_method::route_method::RouteMethod;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_sessions_codegen::sign_out_endpoint_module::SignOutEndpointModule;

use crate::served_session_endpoint::ServedSessionEndpoint;
use crate::session_endpoint_variant::SessionEndpointVariant;
use crate::session_endpoints::SESSION_ENDPOINTS;
use crate::session_endpoints_codegen_error::SessionEndpointsCodegenError;
use crate::session_refresh_route::SessionRefreshRoute;
use crate::session_sign_out_route::SessionSignOutRoute;

fn served_endpoint(
    index: &AttributeIndex,
    anchor: &IndexedItem,
    declared_routes: &DeclaredRoutes,
    variant: SessionEndpointVariant,
    nested: &mut AttributeArgumentsReader,
) -> Result<ServedSessionEndpoint, SessionEndpointsCodegenError> {
    let route = anchor.canonical_path();

    match variant {
        SessionEndpointVariant::Refresh => Ok(ServedSessionEndpoint::Refresh),
        SessionEndpointVariant::SignOut => {
            let written = nested.take_path("landing_route")?.ok_or_else(|| {
                SessionEndpointsCodegenError::MissingLandingRoute {
                    anchor: route.to_string(),
                }
            })?;
            let landing = index.resolve_item_path(anchor, &written).ok_or_else(|| {
                SessionEndpointsCodegenError::UnknownLandingRoute {
                    anchor: route.to_string(),
                    written: format_path(&written),
                }
            })?;

            Ok(ServedSessionEndpoint::SignOut {
                landing: declared_routes.redirect_target(&landing, route)?,
            })
        }
    }
}

pub struct DeclaredSessionEndpoints {
    pub refresh: SessionRefreshRoute,
    pub sign_outs: Vec<SessionSignOutRoute>,
}

impl DeclaredSessionEndpoints {
    /// # Errors
    ///
    /// Returns `SessionEndpointsCodegenError` when a `#[serves_session_endpoint]` declaration is not a
    /// plain routed struct, names no variant of the framework `SessionEndpoint`, answers another
    /// method than POST, lands on a route a redirect cannot reach, serves the refresh endpoint
    /// another route serves, or serves sessions the crate does not issue.
    pub fn read(
        index: &AttributeIndex,
        declared_routes: &DeclaredRoutes,
        sessions: &DeclaredSessions,
    ) -> Result<Self, SessionEndpointsCodegenError> {
        let mut refresh = SessionRefreshRoute::Unserved;
        let mut sign_outs: Vec<SessionSignOutRoute> = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::ServesSessionEndpoint) {
            let declared_anchor =
                declaration_anchor(index, &matched, FrameworkAttribute::ServesSessionEndpoint)?;
            let anchor = declared_anchor.item;
            let route = anchor.canonical_path();
            let endpoint = matched.args()?.interpret(|reader| {
                reader
                    .take_positional_variant(|written, nested| {
                        let variant = index
                            .resolve_item_path(anchor, written)
                            .as_ref()
                            .and_then(|resolved| SESSION_ENDPOINTS.variant(resolved))
                            .ok_or_else(|| {
                                SessionEndpointsCodegenError::UnknownSessionEndpoint {
                                    anchor: route.to_string(),
                                    written: format_path(written),
                                }
                            })?;

                        served_endpoint(index, anchor, declared_routes, variant, nested)
                    })?
                    .ok_or_else(|| SessionEndpointsCodegenError::MissingSessionEndpoint {
                        anchor: route.to_string(),
                    })
            })?;
            let Some(declared) = declared_routes
                .routes
                .iter()
                .find(|declared| declared.item.canonical_path() == route)
            else {
                return Err(SessionEndpointsCodegenError::UnroutedSessionEndpoint {
                    anchor: route.to_string(),
                });
            };

            if declared.method != RouteMethod::Post {
                return Err(SessionEndpointsCodegenError::SessionEndpointMethod {
                    anchor: route.to_string(),
                    method: declared.method,
                });
            }

            if !matches!(sessions, DeclaredSessions::Issued(_)) {
                return Err(
                    SessionEndpointsCodegenError::SessionEndpointWithoutIssuedSessions {
                        anchor: route.to_string(),
                    },
                );
            }

            match (endpoint, &refresh) {
                (ServedSessionEndpoint::Refresh, SessionRefreshRoute::Unserved) => {
                    refresh = SessionRefreshRoute::Served {
                        route: route.clone(),
                    };
                }
                (ServedSessionEndpoint::Refresh, SessionRefreshRoute::Served { route: first }) => {
                    return Err(
                        SessionEndpointsCodegenError::AmbiguousSessionRefreshEndpoint {
                            first: first.to_string(),
                            second: route.to_string(),
                        },
                    );
                }
                (ServedSessionEndpoint::SignOut { landing }, _) => {
                    sign_outs.push(SessionSignOutRoute {
                        handler: SignOutEndpointModule::new(
                            declared_anchor.identifier.field().to_string(),
                        ),
                        landing,
                        route: route.clone(),
                    });
                }
            }
        }

        Ok(Self { refresh, sign_outs })
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_http_codegen::declared_routes::DeclaredRoutes;
    use margaret_http_codegen::http_codegen_error::HttpCodegenError;
    use margaret_route_method::route_method::RouteMethod;
    use margaret_route_parameter_codegen::route_path_registry::RoutePathRegistry;
    use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::DeclaredSessionEndpoints;
    use crate::session_endpoints_codegen_error::SessionEndpointsCodegenError;
    use crate::session_refresh_route::SessionRefreshRoute;
    use crate::session_sign_out_route::SessionSignOutRoute;

    const SESSIONS: &str = "use margaret::framework::macros::issues_sessions;\nuse margaret::framework::macros::issues_tokens;\nuse margaret::framework::macros::responds_to_http;\nuse margaret::framework::macros::serves_session_endpoint;\nuse margaret::framework::macros::singleton;\nuse margaret::framework::route_method::route_method::RouteMethod;\nuse margaret::framework::sessions::session_endpoint::SessionEndpoint;\n\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\nstruct ProviderIssuance;\n\n#[issues_sessions(issuer = provider, audience = \"browser\", cookies = margaret::framework::sessions::session_cookies::SessionCookies::HostOnly)]\nstruct BrowserSessions;\n\n#[singleton]\n#[responds_to_http(method = RouteMethod::Get, path = \"/welcome\", server = \"public\")]\nstruct GetWelcome;\n";

    fn read<TOutcome>(
        source: &str,
        inspect: impl FnOnce(Result<DeclaredSessionEndpoints, SessionEndpointsCodegenError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");
        let sessions = DeclaredSessions::read(&indexed.index, &issuance, &resources)
            .expect("the sessions are read");
        let routes = DeclaredRoutes::read(&indexed.index, &mut RoutePathRegistry::new())
            .expect("the routes are declared");

        inspect(DeclaredSessionEndpoints::read(
            &indexed.index,
            &routes,
            &sessions,
        ))
    }

    fn rejection(source: &str) -> SessionEndpointsCodegenError {
        read(source, |read| read.err().expect("the endpoint is rejected"))
    }

    fn served(endpoint: &str) -> String {
        format!(
            "#[responds_to_http(method = RouteMethod::Post, path = \"/sessions\", server = \"public\")]\n#[serves_session_endpoint({endpoint})]\nstruct PostSessions;\n"
        )
    }

    #[test]
    fn binds_the_refresh_endpoint_to_the_post_route_declaring_it_so_that_route_serves_refreshes() {
        read(
            &format!("{SESSIONS}{}", served("SessionEndpoint::Refresh")),
            |read| {
                let endpoints = read.expect("the endpoint is read");

                assert!(matches!(
                    &endpoints.refresh,
                    SessionRefreshRoute::Served { route } if route.to_string() == "crate::PostSessions"
                ));
                assert!(endpoints.sign_outs.is_empty());
            },
        );
    }

    #[test]
    fn reads_the_sign_out_endpoint_with_its_landing_route() {
        read(
            &format!(
                "{SESSIONS}{}",
                served("SessionEndpoint::SignOut(landing_route = GetWelcome)")
            ),
            |read| {
                let endpoints = read.expect("the endpoint is read");

                assert_eq!(
                    discriminant(&endpoints.refresh),
                    discriminant(&SessionRefreshRoute::Unserved)
                );
                assert!(matches!(
                    endpoints.sign_outs.as_slice(),
                    [SessionSignOutRoute { landing, route, .. }]
                        if landing.path == "/welcome" && landing.server == "public"
                            && route.to_string() == "crate::PostSessions"
                ));
            },
        );
    }

    #[test]
    fn serves_each_sign_out_route_through_a_handler_of_its_own_landing_on_its_own_route() {
        read(
            &format!(
                "{SESSIONS}{}#[singleton]\n#[responds_to_http(method = RouteMethod::Get, path = \"/back-office\", server = \"staff\")]\nstruct GetBackOffice;\n#[responds_to_http(method = RouteMethod::Post, path = \"/staff/sign-out\", server = \"staff\")]\n#[serves_session_endpoint(SessionEndpoint::SignOut(landing_route = GetBackOffice))]\nstruct PostStaffSignOut;\n",
                served("SessionEndpoint::SignOut(landing_route = GetWelcome)")
            ),
            |read| {
                assert!(matches!(
                    read.expect("the endpoints are read").sign_outs.as_slice(),
                    [
                        SessionSignOutRoute { handler: public_handler, landing: public_landing, route: public_route },
                        SessionSignOutRoute { handler: staff_handler, landing: staff_landing, route: staff_route },
                    ] if public_route.to_string() == "crate::PostSessions"
                        && public_landing.server == "public"
                        && public_landing.path == "/welcome"
                        && public_handler.handler_path().to_string() == "crate::margaret::sessions::sign_out_endpoints::post_sessions::session_sign_out_endpoint::SessionSignOutEndpoint"
                        && staff_route.to_string() == "crate::PostStaffSignOut"
                        && staff_landing.server == "staff"
                        && staff_landing.path == "/back-office"
                        && staff_handler.handler_path().to_string() == "crate::margaret::sessions::sign_out_endpoints::post_staff_sign_out::session_sign_out_endpoint::SessionSignOutEndpoint"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_declaration_without_an_endpoint_since_its_session_role_is_unknown() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}{}", served(""))),
            SessionEndpointsCodegenError::MissingSessionEndpoint { anchor } if anchor == "crate::PostSessions"
        ));
    }

    #[test]
    fn rejects_a_variant_of_a_foreign_enum() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}enum Endpoint {{ Refresh }}\n{}", served("Endpoint::Refresh"))),
            SessionEndpointsCodegenError::UnknownSessionEndpoint { written, .. } if written == "Endpoint::Refresh"
        ));
    }

    #[test]
    fn rejects_a_sign_out_without_its_landing_route() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}{}", served("SessionEndpoint::SignOut"))),
            SessionEndpointsCodegenError::MissingLandingRoute { anchor } if anchor == "crate::PostSessions"
        ));
    }

    #[test]
    fn rejects_a_landing_route_that_names_no_item() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}{}", served("SessionEndpoint::SignOut(landing_route = Missing)"))),
            SessionEndpointsCodegenError::UnknownLandingRoute { written, .. } if written == "Missing"
        ));
    }

    #[test]
    fn rejects_a_landing_route_a_redirect_cannot_reach() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}{}", served("SessionEndpoint::SignOut(landing_route = PostSessions)"))),
            SessionEndpointsCodegenError::LandingRoute(HttpCodegenError::RedirectRouteNotGet { referrer, route })
                if referrer == "crate::PostSessions" && route == "crate::PostSessions"
        ));
    }

    #[test]
    fn rejects_a_landing_route_that_is_not_a_path() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}{}", served("SessionEndpoint::SignOut(landing_route = \"/welcome\")"))),
            SessionEndpointsCodegenError::AttributeArguments(AttributeArgumentsError::UnexpectedArgument { key, .. })
                if key == "landing_route"
        ));
    }

    #[test]
    fn rejects_an_endpoint_that_responds_to_no_request() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}#[serves_session_endpoint(SessionEndpoint::Refresh)]\nstruct Refresh;\n")),
            SessionEndpointsCodegenError::UnroutedSessionEndpoint { anchor } if anchor == "crate::Refresh"
        ));
    }

    #[test]
    fn rejects_an_endpoint_answering_another_method_than_post() {
        assert!(matches!(
            rejection(&format!(
                "{SESSIONS}#[responds_to_http(method = RouteMethod::Put, path = \"/sessions\", server = \"public\")]\n#[serves_session_endpoint(SessionEndpoint::Refresh)]\nstruct PutSessions;\n"
            )),
            SessionEndpointsCodegenError::SessionEndpointMethod { anchor, method }
                if anchor == "crate::PutSessions" && method == RouteMethod::Put
        ));
    }

    #[test]
    fn rejects_an_endpoint_of_sessions_the_crate_does_not_issue() {
        assert!(matches!(
            rejection(&format!(
                "use margaret::framework::macros::responds_to_http;\nuse margaret::framework::macros::serves_session_endpoint;\nuse margaret::framework::route_method::route_method::RouteMethod;\nuse margaret::framework::sessions::session_endpoint::SessionEndpoint;\n{}",
                served("SessionEndpoint::Refresh")
            )),
            SessionEndpointsCodegenError::SessionEndpointWithoutIssuedSessions { anchor } if anchor == "crate::PostSessions"
        ));
    }

    #[test]
    fn rejects_the_refresh_endpoint_served_by_two_routes_since_its_url_would_be_ambiguous() {
        assert!(matches!(
            rejection(&format!(
                "{SESSIONS}{}#[responds_to_http(method = RouteMethod::Post, path = \"/again\", server = \"public\")]\n#[serves_session_endpoint(SessionEndpoint::Refresh)]\nstruct PostAgain;\n",
                served("SessionEndpoint::Refresh")
            )),
            SessionEndpointsCodegenError::AmbiguousSessionRefreshEndpoint { first, second }
                if first == "crate::PostAgain" && second == "crate::PostSessions"
        ));
    }

    #[test]
    fn rejects_an_endpoint_declared_on_a_singleton() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}#[singleton]\n{}", served("SessionEndpoint::Refresh"))),
            SessionEndpointsCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. })
                if path == "crate::PostSessions"
        ));
    }

    #[test]
    fn rejects_endpoint_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection(&format!("{SESSIONS}{}", served("= 5"))),
            SessionEndpointsCodegenError::Index(AttributeError::Arguments(AttributeArgumentsError::Malformed { attribute_path, .. }))
                if attribute_path == "serves_session_endpoint"
        ));
    }
}
