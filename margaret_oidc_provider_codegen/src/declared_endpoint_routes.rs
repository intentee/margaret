use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_http_codegen::declared_routes::DeclaredRoutes;

use crate::declared_consent::DeclaredConsent;
use crate::declared_consent_route::DeclaredConsentRoute;
use crate::declared_endpoint_route::DeclaredEndpointRoute;
use crate::endpoint_admission::EndpointAdmission;
use crate::marked_endpoint::MarkedEndpoint;
use crate::oidc_endpoint_marker::OidcEndpointMarker;
use crate::oidc_endpoints::OIDC_ENDPOINTS;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::provider_endpoint::ProviderEndpoint;

fn marker_of(
    index: &AttributeIndex,
    anchor: &IndexedItem,
    marked: MarkedEndpoint,
    nested: &mut AttributeArgumentsReader,
) -> Result<OidcEndpointMarker, OidcProviderCodegenError> {
    let endpoint = |endpoint| Ok(OidcEndpointMarker::Endpoint(endpoint));

    match marked {
        MarkedEndpoint::Authorization => endpoint(ProviderEndpoint::Authorization),
        MarkedEndpoint::Consent => {
            let route = anchor.canonical_path();
            let written = nested.take_path("view")?.ok_or_else(|| {
                OidcProviderCodegenError::MissingConsentView {
                    anchor: route.to_string(),
                }
            })?;
            let view = index.resolve_item_path(anchor, &written).ok_or_else(|| {
                OidcProviderCodegenError::UnknownConsentView {
                    anchor: route.to_string(),
                    written: format_path(&written),
                }
            })?;

            match index.item(&view) {
                Some(rendered)
                    if rendered.has_framework_attribute(FrameworkAttribute::RendersView) =>
                {
                    Ok(OidcEndpointMarker::Consent { view })
                }
                _ => Err(OidcProviderCodegenError::UnrenderedConsentView {
                    anchor: route.to_string(),
                    view: view.to_string(),
                }),
            }
        }
        MarkedEndpoint::Discovery => endpoint(ProviderEndpoint::Discovery),
        MarkedEndpoint::Introspection => endpoint(ProviderEndpoint::Introspection),
        MarkedEndpoint::Jwks => endpoint(ProviderEndpoint::Jwks),
        MarkedEndpoint::Revocation => endpoint(ProviderEndpoint::Revocation),
        MarkedEndpoint::Token => endpoint(ProviderEndpoint::Token),
        MarkedEndpoint::Userinfo => endpoint(ProviderEndpoint::Userinfo),
    }
}

pub struct DeclaredEndpointRoutes {
    pub consent: DeclaredConsent,
    pub routes: Vec<DeclaredEndpointRoute>,
}

impl DeclaredEndpointRoutes {
    /// # Errors
    ///
    /// Returns `OidcProviderCodegenError` when a `#[serves_oidc_endpoint]` declaration is not
    /// anchored by a plain struct, names no variant of the framework `OidcEndpoint`, is not routed
    /// by `#[responds_to_http]`, is routed by a method its endpoint does not admit, renders its
    /// consent page with no view, or declares a second consent endpoint.
    pub fn read(
        index: &AttributeIndex,
        declared_routes: &DeclaredRoutes,
    ) -> Result<Self, OidcProviderCodegenError> {
        let mut consent = DeclaredConsent::Undeclared;
        let mut routes = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::ServesOidcEndpoint) {
            let anchor =
                declaration_anchor(index, &matched, FrameworkAttribute::ServesOidcEndpoint)?.item;
            let route = anchor.canonical_path();
            let marker = matched.args()?.interpret(|reader| {
                reader
                    .take_positional_variant(|variant, nested| {
                        let marked = index
                            .resolve_item_path(anchor, variant)
                            .as_ref()
                            .and_then(|resolved| OIDC_ENDPOINTS.variant(resolved))
                            .ok_or_else(|| OidcProviderCodegenError::UnknownOidcEndpoint {
                                anchor: route.to_string(),
                                written: format_path(variant),
                            })?;

                        marker_of(index, anchor, marked, nested)
                    })?
                    .ok_or_else(|| OidcProviderCodegenError::MissingOidcEndpoint {
                        anchor: route.to_string(),
                    })
            })?;
            let Some(declared) = declared_routes
                .routes
                .iter()
                .find(|declared| declared.item.canonical_path() == route)
            else {
                return Err(OidcProviderCodegenError::UnroutedOidcEndpoint {
                    anchor: route.to_string(),
                });
            };
            let endpoint = marker.endpoint();
            let EndpointAdmission::Admitted(input) = endpoint.admission(declared.method) else {
                return Err(OidcProviderCodegenError::EndpointRouteMethod {
                    endpoint,
                    method: declared.method,
                    path: declared.path.pattern().to_string(),
                });
            };

            match marker {
                OidcEndpointMarker::Consent { view } => {
                    if let DeclaredConsent::Declared(first) = &consent {
                        return Err(OidcProviderCodegenError::AmbiguousConsentEndpoint {
                            first: first.route.to_string(),
                            second: route.to_string(),
                        });
                    }

                    consent = DeclaredConsent::Declared(DeclaredConsentRoute {
                        input,
                        path: declared.path.clone(),
                        route: route.clone(),
                        server: declared.server.clone(),
                        view,
                    });
                }
                OidcEndpointMarker::Endpoint(endpoint) => routes.push(DeclaredEndpointRoute {
                    endpoint,
                    input,
                    path: declared.path.clone(),
                    route: route.clone(),
                    server: declared.server.clone(),
                }),
            }
        }

        Ok(Self { consent, routes })
    }

    #[must_use]
    pub fn serves(&self, endpoint: ProviderEndpoint) -> bool {
        self.routes.iter().any(|route| route.endpoint == endpoint)
    }
}

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
    use margaret_http_codegen::declared_routes::DeclaredRoutes;
    use margaret_http_codegen::framework_input::FrameworkInput;
    use margaret_route_method::route_method::RouteMethod;

    use super::DeclaredEndpointRoutes;
    use crate::declared_consent::DeclaredConsent;
    use crate::declared_consent_route::DeclaredConsentRoute;
    use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
    use crate::provider_endpoint::ProviderEndpoint;

    const CONSENT_VIEW: &str = "use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;\nuse margaret::framework::route_method::route_method::RouteMethod;\nuse crate::views::ConsentView;\n\nmod views {\n    #[renders_view(name = \"consent_view\")]\n    #[singleton]\n    pub struct ConsentView;\n\n    pub struct ConsentLayout;\n}\n\n";

    fn consent_route(endpoint: &str) -> String {
        format!(
            "{CONSENT_VIEW}#[responds_to_http(max_body_bytes = 1_024, method = RouteMethod::Post, path = \"/consent\", server = \"identity\")]\n#[serves_oidc_endpoint({endpoint})]\nstruct PostConsent;\n"
        )
    }

    fn read(source: &str) -> Result<DeclaredEndpointRoutes, OidcProviderCodegenError> {
        let indexed = IndexedSource::new(source);
        let declared = DeclaredRoutes::read(&indexed.index).expect("the routes are declared");

        DeclaredEndpointRoutes::read(&indexed.index, &declared)
    }

    #[test]
    fn rejects_a_consent_view_that_is_not_a_path() {
        assert_eq!(
            read(&consent_route(
                "OidcEndpoint::Consent(view = \"ConsentView\")"
            ))
            .err()
            .expect("the consent view is rejected")
            .to_string(),
            "argument 'view' of attribute 'serves_oidc_endpoint::OidcEndpoint::Consent' is not a path"
        );
    }

    #[test]
    fn reads_an_endpoint_named_through_an_imported_enum() {
        let read = read(
            "use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;\nuse margaret::framework::route_method::route_method::RouteMethod;\n\n#[responds_to_http(max_body_bytes = 1_024, method = RouteMethod::Post, path = \"/token\", server = \"identity\")]\n#[serves_oidc_endpoint(OidcEndpoint::Token)]\nstruct PostToken;\n",
        )
        .expect("the endpoint route is read");

        assert!(matches!(
            read.routes.as_slice(),
            [route] if route.endpoint == ProviderEndpoint::Token
                && route.input == FrameworkInput::Content
                && route.path.pattern() == "/token"
                && route.route.to_string() == "crate::PostToken"
                && route.server == "identity"
        ));
    }

    #[test]
    fn rejects_a_declaration_without_an_endpoint() {
        assert!(matches!(
            read("#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/jwks\", server = \"identity\")]\n#[serves_oidc_endpoint]\nstruct GetJwks;\n"),
            Err(OidcProviderCodegenError::MissingOidcEndpoint { anchor }) if anchor == "crate::GetJwks"
        ));
    }

    #[test]
    fn rejects_a_variant_of_a_foreign_enum() {
        assert!(matches!(
            read("enum OidcEndpoint { Jwks }\n\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/jwks\", server = \"identity\")]\n#[serves_oidc_endpoint(OidcEndpoint::Jwks)]\nstruct GetJwks;\n"),
            Err(OidcProviderCodegenError::UnknownOidcEndpoint { anchor, written })
                if anchor == "crate::GetJwks" && written == "OidcEndpoint::Jwks"
        ));
    }

    #[test]
    fn rejects_an_endpoint_that_responds_to_no_request() {
        assert!(matches!(
            read("#[serves_oidc_endpoint(margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint::Jwks)]\nstruct GetJwks;\n"),
            Err(OidcProviderCodegenError::UnroutedOidcEndpoint { anchor }) if anchor == "crate::GetJwks"
        ));
    }

    #[test]
    fn rejects_an_endpoint_declared_on_a_singleton() {
        assert!(matches!(
            read("#[singleton]\n#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/jwks\", server = \"identity\")]\n#[serves_oidc_endpoint(margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint::Jwks)]\nstruct GetJwks;\n"),
            Err(OidcProviderCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. }))
                if path == "crate::GetJwks"
        ));
    }

    #[test]
    fn rejects_arguments_that_do_not_parse() {
        assert!(matches!(
            read("#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Get, path = \"/jwks\", server = \"identity\")]\n#[serves_oidc_endpoint(= 5)]\nstruct GetJwks;\n"),
            Err(OidcProviderCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            ))) if attribute_path == "serves_oidc_endpoint"
        ));
    }

    #[test]
    fn reads_the_consent_endpoint_with_its_imported_view() {
        let read = read(&consent_route("OidcEndpoint::Consent(view = ConsentView)"))
            .expect("the consent route is read");

        assert!(read.routes.is_empty());
        assert!(matches!(
            read.consent,
            DeclaredConsent::Declared(DeclaredConsentRoute {
                input: FrameworkInput::Content,
                path,
                route,
                server,
                view,
            }) if path.pattern() == "/consent"
                && route.to_string() == "crate::PostConsent"
                && server == "identity"
                && view.to_string() == "crate::views::ConsentView"
        ));
    }

    #[test]
    fn rejects_a_consent_endpoint_without_a_view() {
        assert!(matches!(
            read(&consent_route("OidcEndpoint::Consent")),
            Err(OidcProviderCodegenError::MissingConsentView { anchor }) if anchor == "crate::PostConsent"
        ));
    }

    #[test]
    fn rejects_a_consent_view_that_names_no_item() {
        assert!(matches!(
            read(&consent_route("OidcEndpoint::Consent(view = MissingView)")),
            Err(OidcProviderCodegenError::UnknownConsentView { anchor, written })
                if anchor == "crate::PostConsent" && written == "MissingView"
        ));
    }

    #[test]
    fn rejects_a_consent_view_that_renders_no_view() {
        assert!(matches!(
            read(&consent_route("OidcEndpoint::Consent(view = views::ConsentLayout)")),
            Err(OidcProviderCodegenError::UnrenderedConsentView { anchor, view })
                if anchor == "crate::PostConsent" && view == "crate::views::ConsentLayout"
        ));
    }

    #[test]
    fn rejects_a_second_consent_endpoint() {
        assert!(matches!(
            read(&format!(
                "{}#[responds_to_http(max_body_bytes = 1_024, method = RouteMethod::Post, path = \"/approval\", server = \"identity\")]\n#[serves_oidc_endpoint(OidcEndpoint::Consent(view = ConsentView))]\nstruct PostApproval;\n",
                consent_route("OidcEndpoint::Consent(view = ConsentView)")
            )),
            Err(OidcProviderCodegenError::AmbiguousConsentEndpoint { first, second })
                if first == "crate::PostApproval" && second == "crate::PostConsent"
        ));
    }

    #[test]
    fn rejects_an_endpoint_routed_by_a_method_it_does_not_admit() {
        assert!(matches!(
            read("#[responds_to_http(method = margaret::framework::route_method::route_method::RouteMethod::Put, path = \"/authorize\", server = \"identity\")]\n#[serves_oidc_endpoint(margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint::Authorization)]\nstruct PutAuthorize;\n"),
            Err(OidcProviderCodegenError::EndpointRouteMethod { endpoint, method, path })
                if endpoint == ProviderEndpoint::Authorization
                    && method == RouteMethod::Put
                    && path == "/authorize"
        ));
    }
}
