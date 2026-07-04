use margaret_generated_module::generated_module::GeneratedModule;

use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    modules: Vec<GeneratedModule>,
    servers: Vec<HttpServer>,
}

impl HttpArtifacts {
    pub(crate) fn new(modules: Vec<GeneratedModule>, servers: Vec<HttpServer>) -> Self {
        Self { modules, servers }
    }

    pub fn modules(&self) -> &[GeneratedModule] {
        &self.modules
    }

    pub fn servers(&self) -> &[HttpServer] {
        &self.servers
    }
}
