use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[derive(Debug)]
pub struct ConsoleArtifacts {
    pub module: GeneratedModuleTokens,
    pub construction_roots: Vec<CanonicalPath>,
}
