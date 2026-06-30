use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    servers: Vec<HttpServer>,
    source: String,
}

impl HttpArtifacts {
    pub(crate) fn new(source: String, servers: Vec<HttpServer>) -> Self {
        Self { servers, source }
    }

    pub fn servers(&self) -> &[HttpServer] {
        &self.servers
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
