use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    pub retained_roots: Vec<CanonicalPath>,
    pub modules: Vec<GeneratedModuleTokens>,
    pub servers: Vec<HttpServer>,
}

impl HttpArtifacts {
    pub(crate) fn new(
        modules: Vec<GeneratedModuleTokens>,
        servers: Vec<HttpServer>,
        retained_roots: Vec<CanonicalPath>,
    ) -> Self {
        Self {
            retained_roots,
            modules,
            servers,
        }
    }

    #[must_use]
    pub fn into_modules(self) -> Vec<GeneratedModuleTokens> {
        self.modules
    }

    #[must_use]
    pub fn servers(&self) -> &[HttpServer] {
        &self.servers
    }
}
