use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[derive(Debug)]
pub struct ViewsArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub retained_roots: Vec<CanonicalPath>,
}
