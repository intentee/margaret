use std::collections::BTreeMap;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::http_server::HttpServer;

pub struct HttpArtifacts {
    modules: Vec<GeneratedModuleTokens>,
    server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    servers: Vec<HttpServer>,
}

impl HttpArtifacts {
    pub(crate) fn new(
        modules: Vec<GeneratedModuleTokens>,
        servers: Vec<HttpServer>,
        server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    ) -> Self {
        Self {
            modules,
            server_console_arguments,
            servers,
        }
    }

    #[must_use]
    pub fn into_modules(self) -> Vec<GeneratedModuleTokens> {
        self.modules
    }

    #[must_use]
    pub fn server_console_arguments(&self) -> &BTreeMap<String, Vec<ConsoleArgument>> {
        &self.server_console_arguments
    }

    #[must_use]
    pub fn servers(&self) -> &[HttpServer] {
        &self.servers
    }
}
