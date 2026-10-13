use margaret_oidc_provider_codegen::render_provider_endpoints::render_provider_endpoints;

use crate::provided_endpoints::ProvidedEndpoints;
use crate::provider_endpoint_artifacts::ProviderEndpointArtifacts;
use crate::provider_server::ProviderServer;

pub(crate) fn provider_endpoint_modules(provided: &ProvidedEndpoints) -> ProviderEndpointArtifacts {
    match provided {
        ProvidedEndpoints::Derived(endpoints) => ProviderEndpointArtifacts {
            modules: render_provider_endpoints(endpoints),
            server: ProviderServer::Named(endpoints.server.clone()),
        },
        ProvidedEndpoints::Unprovided => ProviderEndpointArtifacts {
            modules: Vec::new(),
            server: ProviderServer::Absent,
        },
    }
}
