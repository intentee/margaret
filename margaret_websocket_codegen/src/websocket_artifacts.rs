use margaret_codegen_tokens::synthetic_route::SyntheticRoute;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[derive(Debug)]
pub struct WebsocketArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub synthetic_routes: Vec<SyntheticRoute>,
}
