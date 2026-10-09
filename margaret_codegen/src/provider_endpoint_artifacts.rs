use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::provider_server::ProviderServer;

pub(crate) struct ProviderEndpointArtifacts {
    pub(crate) modules: Vec<GeneratedModuleTokens>,
    pub(crate) server: ProviderServer,
}
