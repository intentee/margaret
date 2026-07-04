use margaret_attributes::attribute_index::AttributeIndex;
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
    servers: Vec<HttpServer>,
}

impl<'index> BuildContext<'index> {
    pub(crate) fn new(index: &'index AttributeIndex) -> Self {
        Self {
            capabilities: Capabilities::detect(index),
            index,
            module_tokens: Vec::new(),
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

    pub(crate) fn servers(&self) -> &[HttpServer] {
        &self.servers
    }

    pub(crate) fn set_servers(&mut self, servers: Vec<HttpServer>) {
        self.servers = servers;
    }
}
