use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_oidc_provider_codegen::derive_provider_endpoint_routes::derive_provider_endpoint_routes;
use margaret_oidc_provider_codegen::oidc_provider_codegen_error::OidcProviderCodegenError;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_oidc_provider_codegen::render_provider_endpoint_paths::render_provider_endpoint_paths;
use margaret_service_codegen::served_origin_check::ServedOriginCheck;

use crate::provider_endpoint_artifacts::ProviderEndpointArtifacts;

pub(crate) fn provider_endpoint_modules(
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
) -> Result<ProviderEndpointArtifacts, OidcProviderCodegenError> {
    let provider_endpoints_path = oidc_provider_item_path(OidcProviderItem::ProviderEndpoints);

    match bindings.provider(&provider_endpoints_path) {
        Some(provider_endpoints) => {
            derive_provider_endpoint_routes(locations, bindings).map(|routes| {
                ProviderEndpointArtifacts {
                    checks: vec![ServedOriginCheck {
                        accessor_field: provider_endpoints.field_name.clone(),
                        server: routes.server.clone(),
                    }],
                    modules: vec![render_provider_endpoint_paths(&routes)],
                    retained_roots: vec![provider_endpoints_path],
                }
            })
        }
        None => Ok(ProviderEndpointArtifacts {
            checks: Vec::new(),
            modules: Vec::new(),
            retained_roots: Vec::new(),
        }),
    }
}
