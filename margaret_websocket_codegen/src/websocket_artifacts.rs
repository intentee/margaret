use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[derive(Debug)]
pub struct WebSocketArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub servers: Vec<String>,
}
