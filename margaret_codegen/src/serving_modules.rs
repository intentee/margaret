use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;

pub(crate) struct ServingModules {
    pub(crate) modules: Vec<GeneratedModuleTokens>,
    pub(crate) retained_roots: Vec<CanonicalPath>,
    pub(crate) servers: Vec<HttpServer>,
}
