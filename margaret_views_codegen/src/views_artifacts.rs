use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_serve_input_codegen::serve_input::ServeInput;

#[derive(Debug)]
pub struct ViewsArtifacts {
    pub serve_inputs: Vec<ServeInput>,
    pub modules: Vec<GeneratedModuleTokens>,
    pub retained_roots: Vec<CanonicalPath>,
}
