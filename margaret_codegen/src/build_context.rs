use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;

use crate::capabilities::Capabilities;
use crate::format_pass::format_pass;
use crate::generated_code::GeneratedCode;
use crate::umbrella::umbrella;

pub(crate) struct BuildContext<'index> {
    capabilities: Capabilities,
    index: &'index AttributeIndex,
    module_tokens: Vec<GeneratedModuleTokens>,
    serve_arguments: Vec<ConsoleArgument>,
    servers: Vec<HttpServer>,
}

impl<'index> BuildContext<'index> {
    pub(crate) fn new(index: &'index AttributeIndex) -> Self {
        Self {
            capabilities: Capabilities::detect(index),
            index,
            module_tokens: Vec::new(),
            serve_arguments: Vec::new(),
            servers: Vec::new(),
        }
    }

    pub(crate) fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    pub(crate) fn extend_modules(&mut self, modules: Vec<GeneratedModuleTokens>) {
        self.module_tokens.extend(modules);
    }

    pub(crate) fn index(&self) -> &'index AttributeIndex {
        self.index
    }

    pub(crate) fn into_generated_code(self) -> GeneratedCode {
        let mut modules = format_pass(self.module_tokens);

        modules.push(umbrella(self.capabilities));

        GeneratedCode::new(modules)
    }

    #[cfg(test)]
    pub(crate) fn module_tokens(&self) -> &[GeneratedModuleTokens] {
        &self.module_tokens
    }

    pub(crate) fn serve_arguments(&self) -> &[ConsoleArgument] {
        &self.serve_arguments
    }

    pub(crate) fn servers(&self) -> &[HttpServer] {
        &self.servers
    }

    pub(crate) fn set_serve_arguments(&mut self, serve_arguments: Vec<ConsoleArgument>) {
        self.serve_arguments = serve_arguments;
    }

    pub(crate) fn set_servers(&mut self, servers: Vec<HttpServer>) {
        self.servers = servers;
    }
}
