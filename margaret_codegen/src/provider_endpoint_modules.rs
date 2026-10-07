use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_oidc_provider_codegen::derive_provider_endpoints::derive_provider_endpoints;
use margaret_oidc_provider_codegen::oidc_provider_codegen_error::OidcProviderCodegenError;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_oidc_provider_codegen::provider_endpoints_path::provider_endpoints_path;
use margaret_oidc_provider_codegen::render_provider_endpoints::render_provider_endpoints;
use margaret_service_codegen::served_origin_check::ServedOriginCheck;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::provider_endpoint_artifacts::ProviderEndpointArtifacts;

pub(crate) fn provider_endpoint_modules(
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
    token_issuance: &DeclaredTokenIssuance,
) -> Result<ProviderEndpointArtifacts, OidcProviderCodegenError> {
    let uses_provider_endpoints = OidcProviderItem::ALL
        .into_iter()
        .filter(|item| item.uses_provider_endpoints())
        .any(|item| bindings.provides(&oidc_provider_item_path(item)));

    match token_issuance {
        DeclaredTokenIssuance::Declared(declaration) if uses_provider_endpoints => {
            derive_provider_endpoints(locations, bindings, declaration).map(|endpoints| {
                ProviderEndpointArtifacts {
                    checks: vec![ServedOriginCheck {
                        endpoints: provider_endpoints_path(),
                        server: endpoints.server.clone(),
                    }],
                    modules: vec![render_provider_endpoints(&endpoints)],
                }
            })
        }
        DeclaredTokenIssuance::Absent | DeclaredTokenIssuance::Declared(_) => {
            Ok(ProviderEndpointArtifacts {
                checks: Vec::new(),
                modules: Vec::new(),
            })
        }
    }
}
