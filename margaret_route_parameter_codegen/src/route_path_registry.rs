use std::collections::BTreeMap;

use crate::route_path::RoutePath;
use crate::route_path_error::RoutePathError;
use crate::server_route_paths::ServerRoutePaths;

#[derive(Default)]
pub struct RoutePathRegistry {
    servers: BTreeMap<String, ServerRoutePaths>,
}

impl RoutePathRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Errors
    ///
    /// Returns `RoutePathError` when the router of the server cannot route the pattern, the
    /// pattern conflicts with a path admitted before, a websocket session already serves it, or
    /// the pattern breaks a rule of Margaret route paths.
    pub fn admit_responder_path(
        &mut self,
        server: &str,
        pattern: &str,
    ) -> Result<RoutePath, RoutePathError> {
        self.server_paths(server).admit_responder_path(pattern)
    }

    /// # Errors
    ///
    /// Returns `RoutePathError` when the router of the server cannot route the pattern, the
    /// pattern conflicts with a path admitted before, a responder already serves it, or the
    /// pattern breaks a rule of Margaret route paths.
    pub fn admit_web_socket_session_path(
        &mut self,
        server: &str,
        pattern: &str,
    ) -> Result<RoutePath, RoutePathError> {
        self.server_paths(server)
            .admit_web_socket_session_path(pattern)
    }

    fn server_paths(&mut self, server: &str) -> &mut ServerRoutePaths {
        self.servers.entry(server.to_owned()).or_default()
    }
}

#[cfg(test)]
mod tests {
    use matchit::InsertError;

    use super::RoutePathRegistry;
    use crate::route_path_error::RoutePathError;

    #[test]
    fn admits_a_responder_path_returning_its_pattern_unchanged_for_the_router_to_register() {
        assert_eq!(
            RoutePathRegistry::new()
                .admit_responder_path("public", "/articles/{article}")
                .expect("the path is admitted")
                .pattern(),
            "/articles/{article}"
        );
    }

    #[test]
    fn admits_one_responder_path_for_every_method_that_serves_it() {
        let mut registry = RoutePathRegistry::new();

        registry
            .admit_responder_path("public", "/articles")
            .expect("the path is admitted");

        assert!(registry.admit_responder_path("public", "/articles").is_ok());
    }

    #[test]
    fn admits_one_path_on_different_servers_since_each_server_routes_its_own_requests() {
        let mut registry = RoutePathRegistry::new();

        registry
            .admit_responder_path("public", "/articles")
            .expect("the path is admitted");

        assert!(
            registry
                .admit_web_socket_session_path("internal", "/articles")
                .is_ok()
        );
    }

    #[test]
    fn rejects_a_path_conflicting_with_an_admitted_one_since_a_request_could_match_both() {
        let mut registry = RoutePathRegistry::new();

        registry
            .admit_responder_path("public", "/articles/{article}")
            .expect("the path is admitted");

        assert_eq!(
            registry
                .admit_responder_path("public", "/articles/{id}")
                .err(),
            Some(RoutePathError::ConflictingRoutePath {
                conflicting_path: "/articles/{article}".to_string(),
            })
        );
    }

    #[test]
    fn rejects_an_unterminated_parameter_at_compile_time_since_the_router_cannot_parse_it() {
        assert_eq!(
            router_rejection("/users/{id"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidParam,
            })
        );
    }

    fn router_rejection(pattern: &str) -> Option<RoutePathError> {
        RoutePathRegistry::new()
            .admit_responder_path("public", pattern)
            .err()
    }

    #[test]
    fn rejects_text_after_a_parameter_because_the_router_cannot_route_it() {
        assert_eq!(
            router_rejection("/images/{id}.png"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidParamSegment,
            })
        );
    }

    #[test]
    fn rejects_two_parameters_in_one_segment_because_the_router_cannot_route_them() {
        assert_eq!(
            router_rejection("/{first}-{second}"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidParamSegment,
            })
        );
    }

    #[test]
    fn rejects_a_control_character_after_a_parameter_the_router_cannot_route() {
        assert_eq!(
            router_rejection("/{id}\t"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidParamSegment,
            })
        );
    }

    #[test]
    fn rejects_a_catch_all_before_another_segment_the_router_cannot_route() {
        assert_eq!(
            router_rejection("/{*rest}/tail"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidCatchAll,
            })
        );
    }

    #[test]
    fn rejects_a_catch_all_followed_by_literal_text_the_router_cannot_route() {
        assert_eq!(
            router_rejection("/{*rest}.png"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidParamSegment,
            })
        );
    }

    #[test]
    fn rejects_an_unmatched_closing_brace_the_router_cannot_parse() {
        assert_eq!(
            router_rejection("/a}b"),
            Some(RoutePathError::RejectedByRouter {
                source: InsertError::InvalidParam,
            })
        );
    }

    #[test]
    fn rejects_a_responder_at_a_web_socket_session_path_since_one_path_has_one_handler() {
        let mut registry = RoutePathRegistry::new();

        registry
            .admit_web_socket_session_path("public", "/chat")
            .expect("the path is admitted");

        assert_eq!(
            registry.admit_responder_path("public", "/chat").err(),
            Some(RoutePathError::ServedByWebSocketSession)
        );
    }

    #[test]
    fn rejects_a_web_socket_session_at_a_responder_path_since_one_path_has_one_handler() {
        let mut registry = RoutePathRegistry::new();

        registry
            .admit_responder_path("public", "/chat")
            .expect("the path is admitted");

        assert_eq!(
            registry
                .admit_web_socket_session_path("public", "/chat")
                .err(),
            Some(RoutePathError::ServedByResponder)
        );
    }

    #[test]
    fn rejects_two_web_socket_sessions_at_one_path_since_a_request_could_reach_either() {
        let mut registry = RoutePathRegistry::new();

        registry
            .admit_web_socket_session_path("public", "/chat")
            .expect("the path is admitted");

        assert_eq!(
            registry
                .admit_web_socket_session_path("public", "/chat")
                .err(),
            Some(RoutePathError::ConflictingRoutePath {
                conflicting_path: "/chat".to_string(),
            })
        );
    }

    #[test]
    fn rejects_a_pattern_the_router_accepts_when_it_declares_a_parameter_twice() {
        assert_eq!(
            RoutePathRegistry::new()
                .admit_responder_path("public", "/{id}/{id}")
                .err(),
            Some(RoutePathError::DuplicateParameterName {
                name: "id".to_string(),
            })
        );
    }
}
