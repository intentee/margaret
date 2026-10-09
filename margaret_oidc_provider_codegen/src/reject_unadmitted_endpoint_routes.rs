use crate::declared_consent::DeclaredConsent;
use crate::declared_endpoint_routes::DeclaredEndpointRoutes;
use crate::endpoint_routes::EndpointRoutes;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::provider_endpoint::ProviderEndpoint;

const ADMISSION_ENDPOINTS: [ProviderEndpoint; 6] = [
    ProviderEndpoint::Authorization,
    ProviderEndpoint::Discovery,
    ProviderEndpoint::Introspection,
    ProviderEndpoint::Revocation,
    ProviderEndpoint::Token,
    ProviderEndpoint::Userinfo,
];

/// # Errors
///
/// Returns `OidcProviderCodegenError::EndpointRouteWithoutAdmittedClients` when a route serves an
/// endpoint that only admitted oauth clients use.
pub fn reject_unadmitted_endpoint_routes(
    marked: &DeclaredEndpointRoutes,
) -> Result<(), OidcProviderCodegenError> {
    let routes = EndpointRoutes { marked };

    match (
        ADMISSION_ENDPOINTS
            .into_iter()
            .find(|endpoint| !routes.serving(*endpoint).is_empty()),
        &marked.consent,
    ) {
        (Some(endpoint), _) => {
            Err(OidcProviderCodegenError::EndpointRouteWithoutAdmittedClients { endpoint })
        }
        (None, DeclaredConsent::Declared(_)) => Err(
            OidcProviderCodegenError::EndpointRouteWithoutAdmittedClients {
                endpoint: ProviderEndpoint::Consent,
            },
        ),
        (None, DeclaredConsent::Undeclared) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_http_codegen::declared_routes::DeclaredRoutes;

    use super::reject_unadmitted_endpoint_routes;
    use crate::declared_endpoint_routes::DeclaredEndpointRoutes;

    #[test]
    fn rejects_a_consent_route_of_a_crate_that_admits_no_clients() {
        let indexed = IndexedSource::new(
            "use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;\nuse margaret::framework::route_method::route_method::RouteMethod;\n\n#[renders_view(name = \"consent_view\")]\n#[singleton]\npub struct ConsentView;\n\n#[responds_to_http(max_body_bytes = 1_024, method = RouteMethod::Post, path = \"/consent\", server = \"identity\")]\n#[serves_oidc_endpoint(OidcEndpoint::Consent(view = ConsentView))]\nstruct PostConsent;\n",
        );
        let declared = DeclaredRoutes::read(&indexed.index).expect("the routes are declared");

        assert_eq!(
            reject_unadmitted_endpoint_routes(
                &DeclaredEndpointRoutes::read(&indexed.index, &declared)
                    .expect("the endpoint routes are read")
            )
            .expect_err("the consent route is rejected")
            .to_string(),
            "the consent endpoint is routed, but the provider admits no oauth client"
        );
    }
}
