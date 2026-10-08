use margaret_http_codegen::http_artifacts::HttpArtifacts;

use crate::provider_endpoint_artifacts::ProviderEndpointArtifacts;

pub(crate) struct HttpModules {
    pub(crate) artifacts: HttpArtifacts,
    pub(crate) provider_endpoints: ProviderEndpointArtifacts,
}
