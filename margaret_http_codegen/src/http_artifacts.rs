use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    pub retained_roots: Vec<CanonicalPath>,
    pub modules: Vec<GeneratedModuleTokens>,
    pub servers: Vec<HttpServer>,
}

impl HttpArtifacts {
    #[must_use]
    pub fn into_modules(self) -> Vec<GeneratedModuleTokens> {
        self.modules
    }

    #[must_use]
    pub fn servers(&self) -> &[HttpServer] {
        &self.servers
    }
}
