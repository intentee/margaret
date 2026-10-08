use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;

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
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
) -> Result<(), OidcProviderCodegenError> {
    let routes = EndpointRoutes {
        bindings,
        locations,
    };

    match ADMISSION_ENDPOINTS
        .into_iter()
        .find(|endpoint| !routes.serving(*endpoint).is_empty())
    {
        Some(endpoint) => {
            Err(OidcProviderCodegenError::EndpointRouteWithoutAdmittedClients { endpoint })
        }
        None => Ok(()),
    }
}
