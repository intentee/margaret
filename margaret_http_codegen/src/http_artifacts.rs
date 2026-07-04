use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    modules: Vec<GeneratedModuleTokens>,
    servers: Vec<HttpServer>,
}

impl HttpArtifacts {
    pub(crate) fn new(modules: Vec<GeneratedModuleTokens>, servers: Vec<HttpServer>) -> Self {
        Self { modules, servers }
    }

    pub fn into_modules(self) -> Vec<GeneratedModuleTokens> {
        self.modules
    }

    pub fn servers(&self) -> &[HttpServer] {
        &self.servers
    }
}
