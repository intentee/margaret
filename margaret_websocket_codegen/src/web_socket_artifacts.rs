use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_serve_input_codegen::serve_input::ServeInput;

#[derive(Debug)]
pub struct WebSocketArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub retained_roots: Vec<CanonicalPath>,
    pub server_serve_inputs: BTreeMap<String, Vec<ServeInput>>,
    pub servers: Vec<String>,
}
