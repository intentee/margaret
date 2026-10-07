use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_service_codegen::served_origin_check::ServedOriginCheck;

pub(crate) struct ProviderEndpointArtifacts {
    pub(crate) checks: Vec<ServedOriginCheck>,
    pub(crate) modules: Vec<GeneratedModuleTokens>,
}
