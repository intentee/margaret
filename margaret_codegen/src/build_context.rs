use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;

use crate::capabilities::Capabilities;
use crate::codegen_error::CodegenError;
use crate::format_pass::format_pass;
use crate::generated_code::GeneratedCode;
use crate::umbrella::umbrella;

pub(crate) struct BuildContext<'index> {
    capabilities: Capabilities,
    index: &'index AttributeIndex,
    module_tokens: Vec<GeneratedModuleTokens>,
    provided_singletons: Vec<ProvidedSingleton>,
    serve_arguments: Vec<ConsoleArgument>,
    servers: Vec<HttpServer>,
    websocket_servers: Vec<String>,
}

impl<'index> BuildContext<'index> {
    pub(crate) fn new(
        index: &'index AttributeIndex,
        provided_singletons: Vec<ProvidedSingleton>,
    ) -> Self {
        Self {
            capabilities: Capabilities::detect(index),
            index,
            module_tokens: Vec::new(),
            provided_singletons,
            serve_arguments: Vec::new(),
            servers: Vec::new(),
            websocket_servers: Vec::new(),
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

    pub(crate) fn into_generated_code(self) -> Result<GeneratedCode, CodegenError> {
        let mut modules = format_pass(self.module_tokens)?;

        modules.push(umbrella(self.capabilities));

        Ok(GeneratedCode::new(modules))
    }

    #[cfg(test)]
    pub(crate) fn module_tokens(&self) -> &[GeneratedModuleTokens] {
        &self.module_tokens
    }

    pub(crate) fn provided_singletons(&self) -> &[ProvidedSingleton] {
        &self.provided_singletons
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

    pub(crate) fn set_websocket_servers(&mut self, websocket_servers: Vec<String>) {
        self.websocket_servers = websocket_servers;
    }

    pub(crate) fn websocket_servers(&self) -> &[String] {
        &self.websocket_servers
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use quote::quote;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::BuildContext;

    #[test]
    fn rejects_a_generated_module_that_is_not_a_valid_file() {
        let directory = tempdir().expect("a temporary crate directory");
        let source = directory.path().join("src");
        fs::create_dir_all(&source).expect("the src directory exists");
        fs::write(source.join("lib.rs"), "").expect("lib.rs is written");

        let index = AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source))
            .expect("the crate is indexed")
            .build();
        let mut context = BuildContext::new(&index, Vec::new());
        context.extend_modules(vec![GeneratedModuleTokens::new("broken", quote! { fn })]);

        let error = context
            .into_generated_code()
            .expect_err("an invalid generated module is rejected");

        assert!(error.to_string().contains("failed to format a generated module"));
    }
}
