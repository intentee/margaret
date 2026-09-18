use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_server_codegen::server_contribution::ServerContribution;

#[derive(Debug)]
pub struct WebSocketArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub retained_roots: Vec<CanonicalPath>,
    pub server_contributions: Vec<ServerContribution>,
}
