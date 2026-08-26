use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    pub retained_roots: Vec<CanonicalPath>,
    pub modules: Vec<GeneratedModuleTokens>,
    pub server_serve_inputs: BTreeMap<String, Vec<ServeInput>>,
    pub servers: Vec<HttpServer>,
}

impl HttpArtifacts {
    pub(crate) fn new(
        modules: Vec<GeneratedModuleTokens>,
        servers: Vec<HttpServer>,
        server_serve_inputs: BTreeMap<String, Vec<ServeInput>>,
        retained_roots: Vec<CanonicalPath>,
    ) -> Self {
        Self {
            retained_roots,
            modules,
            server_serve_inputs,
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
