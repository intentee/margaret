use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_path::RoutePath;
use margaret_route_parameter_codegen::route_url_template::RouteUrlTemplate;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

use crate::declared_route::DeclaredRoute;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;

pub struct DeclaredRoutes<'index> {
    pub routes: Vec<DeclaredRoute<'index>>,
}

impl<'index> DeclaredRoutes<'index> {
    /// # Errors
    ///
    /// Returns `HttpCodegenError` when a `#[responds_to_http]` declaration is malformed or two
    /// routes share a name.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, HttpCodegenError> {
        let mut routes = Vec::new();
        let mut seen_names: HashMap<String, String> = HashMap::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::RespondsToHttp) {
            let item = matched.item();
            let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
                return Err(HttpCodegenError::RespondsToHttpNotOnStruct {
                    target: item.canonical_path().to_string(),
                });
            };
            let responder = item.canonical_path().to_string();
            let HttpResponderArguments {
                max_body_bytes,
                method,
                name,
                path,
                server,
            } = HttpResponderArguments::parse(matched.args()?, index, item, &responder)?;

            if !is_snake_case_identifier(&server) {
                return Err(HttpCodegenError::InvalidServerName { responder, server });
            }

            let path = match RoutePath::parse(&path) {
                Ok(parsed) => parsed,
                Err(source) => {
                    return Err(HttpCodegenError::UnroutableRoutePath {
                        path,
                        responder,
                        source,
                    });
                }
            };

            if let Some(name) = &name {
                if !is_snake_case_identifier(name) {
                    return Err(HttpCodegenError::InvalidRouteName {
                        name: name.clone(),
                        responder,
                    });
                }

                if let Some(first) = seen_names.get(name) {
                    return Err(HttpCodegenError::DuplicateRouteName {
                        name: name.clone(),
                        first: first.clone(),
                        second: responder,
                    });
                }

                seen_names.insert(name.clone(), responder);
            }

            routes.push(DeclaredRoute {
                identifier,
                item,
                max_body_bytes,
                method,
                name,
                path,
                server,
            });
        }

        Ok(Self { routes })
    }

    /// # Errors
    ///
    /// Returns `HttpCodegenError::RedirectRouteNotRouted` for an item that responds to no HTTP
    /// request, `HttpCodegenError::RedirectRouteNotGet` for a route of another method and
    /// `HttpCodegenError::ParameterizedRedirectRoute` for a route whose path has parameters.
    pub fn redirect_target(
        &self,
        route: &CanonicalPath,
        referrer: &CanonicalPath,
    ) -> Result<RouteUrlInput, HttpCodegenError> {
        let Some(declared) = self
            .routes
            .iter()
            .find(|declared| declared.item.canonical_path() == route)
        else {
            return Err(HttpCodegenError::RedirectRouteNotRouted {
                referrer: referrer.to_string(),
                route: route.to_string(),
            });
        };

        if declared.method != RouteMethod::Get {
            return Err(HttpCodegenError::RedirectRouteNotGet {
                referrer: referrer.to_string(),
                route: route.to_string(),
            });
        }

        match declared.path.template() {
            RouteUrlTemplate::Literal(path) => Ok(RouteUrlInput {
                path: path.clone(),
                server: declared.server.clone(),
            }),
            RouteUrlTemplate::Parameterized(_) => {
                Err(HttpCodegenError::ParameterizedRedirectRoute {
                    path: declared.path.pattern().to_string(),
                    referrer: referrer.to_string(),
                    route: route.to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

    use super::DeclaredRoutes;
    use crate::http_codegen_error::HttpCodegenError;

    const ROUTES: &str = "use margaret::framework::route_method::route_method::RouteMethod;\n\n#[responds_to_http(method = RouteMethod::Get, path = \"/sign-in/callback\", server = \"public\")]\nstruct Callback;\n#[responds_to_http(method = RouteMethod::Post, path = \"/sign-in\", server = \"public\")]\nstruct PostSignIn;\n#[responds_to_http(method = RouteMethod::Get, path = \"/articles/{article}\", server = \"public\")]\nstruct Article;\n#[responds_to_http(method = RouteMethod::Get, path = \"/oauth/{{callback}}\", server = \"public\")]\nstruct EscapedCallback;\nstruct Unrouted;\n";

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn redirect_target(route: &str) -> Result<RouteUrlInput, HttpCodegenError> {
        let indexed = IndexedSource::new(ROUTES);

        DeclaredRoutes::read(&indexed.index)
            .expect("the routes are declared")
            .redirect_target(&path(route), &path("Client"))
    }

    fn rejection(route: &str) -> String {
        redirect_target(route)
            .expect_err("the redirect route is rejected")
            .to_string()
    }

    #[test]
    fn locates_a_redirect_route_by_its_server_and_path() {
        assert_eq!(
            redirect_target("Callback").expect("the redirect route is routed"),
            RouteUrlInput {
                path: "/sign-in/callback".to_string(),
                server: "public".to_string(),
            }
        );
    }

    #[test]
    fn locates_a_redirect_route_with_escaped_braces_at_its_encoded_literal_path() {
        assert_eq!(
            redirect_target("EscapedCallback").expect("the redirect route is routed"),
            RouteUrlInput {
                path: "/oauth/%7Bcallback%7D".to_string(),
                server: "public".to_string(),
            }
        );
    }

    #[test]
    fn rejects_a_redirect_route_that_responds_to_no_request() {
        assert_eq!(
            rejection("Unrouted"),
            "'crate::Client' redirects to 'crate::Unrouted', which responds to no HTTP request"
        );
    }

    #[test]
    fn rejects_a_redirect_route_of_another_method() {
        assert_eq!(
            rejection("PostSignIn"),
            "'crate::Client' redirects to 'crate::PostSignIn', which does not respond to RouteMethod::Get"
        );
    }

    #[test]
    fn rejects_a_route_path_no_request_can_reach() {
        let indexed = IndexedSource::new(
            "use margaret::framework::route_method::route_method::RouteMethod;\n\n#[responds_to_http(method = RouteMethod::Get, path = \"/articles/../admin\", server = \"public\")]\nstruct Escaping;\n",
        );

        assert!(matches!(
            DeclaredRoutes::read(&indexed.index),
            Err(HttpCodegenError::UnroutableRoutePath { path, responder, .. })
                if path == "/articles/../admin" && responder == "crate::Escaping"
        ));
    }

    #[test]
    fn rejects_a_redirect_route_with_parameters() {
        assert_eq!(
            rejection("Article"),
            "'crate::Client' redirects to 'crate::Article', whose path '/articles/{article}' has parameters a redirect cannot fill"
        );
    }
}
