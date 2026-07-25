use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_service_codegen::framework_service::FrameworkService;

use crate::capabilities::Capabilities;
use crate::codegen_error::CodegenError;
use crate::format_pass::format_pass;
use crate::generated_code::GeneratedCode;
use crate::umbrella::umbrella;

pub(crate) struct BuildContext<'index> {
    capabilities: Capabilities,
    framework_services: Vec<FrameworkService>,
    index: &'index AttributeIndex,
    metafile_contents: Option<String>,
    middleware_plans: Vec<MiddlewarePlan>,
    module_tokens: Vec<GeneratedModuleTokens>,
    server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    servers: Vec<HttpServer>,
    views_console_arguments: Vec<ConsoleArgument>,
    websocket_server_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    websocket_servers: Vec<String>,
}

impl<'index> BuildContext<'index> {
    pub(crate) fn new(index: &'index AttributeIndex, metafile_contents: Option<String>) -> Self {
        Self {
            capabilities: Capabilities::detect(index, metafile_contents.is_some()),
            framework_services: Vec::new(),
            index,
            metafile_contents,
            middleware_plans: Vec::new(),
            module_tokens: Vec::new(),
            server_console_arguments: BTreeMap::new(),
            servers: Vec::new(),
            views_console_arguments: Vec::new(),
            websocket_server_arguments: BTreeMap::new(),
            websocket_servers: Vec::new(),
        }
    }

    pub(crate) fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    pub(crate) fn enable_jwks(&mut self) {
        self.capabilities.has_jwks = true;
    }

    pub(crate) fn enable_postgres_pool(&mut self) {
        self.capabilities.has_postgres_pool = true;
    }

    pub(crate) fn extend_modules(&mut self, modules: Vec<GeneratedModuleTokens>) {
        self.module_tokens.extend(modules);
    }

    pub(crate) fn framework_services(&self) -> &[FrameworkService] {
        &self.framework_services
    }

    pub(crate) fn index(&self) -> &'index AttributeIndex {
        self.index
    }

    pub(crate) fn metafile_contents(&self) -> Option<&str> {
        self.metafile_contents.as_deref()
    }

    pub(crate) fn middleware_plans(&self) -> &[MiddlewarePlan] {
        &self.middleware_plans
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

    pub(crate) fn server_console_arguments(&self) -> &BTreeMap<String, Vec<ConsoleArgument>> {
        &self.server_console_arguments
    }

    pub(crate) fn servers(&self) -> &[HttpServer] {
        &self.servers
    }

    pub(crate) fn set_framework_services(&mut self, framework_services: Vec<FrameworkService>) {
        self.framework_services = framework_services;
    }

    pub(crate) fn set_middleware_plans(&mut self, middleware_plans: Vec<MiddlewarePlan>) {
        self.middleware_plans = middleware_plans;
    }

    pub(crate) fn set_server_console_arguments(
        &mut self,
        server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    ) {
        self.server_console_arguments = server_console_arguments;
    }

    pub(crate) fn set_servers(&mut self, servers: Vec<HttpServer>) {
        self.servers = servers;
    }

    pub(crate) fn set_views_console_arguments(
        &mut self,
        views_console_arguments: Vec<ConsoleArgument>,
    ) {
        self.views_console_arguments = views_console_arguments;
    }

    pub(crate) fn set_websocket_server_arguments(
        &mut self,
        websocket_server_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    ) {
        self.websocket_server_arguments = websocket_server_arguments;
    }

    pub(crate) fn set_websocket_servers(&mut self, websocket_servers: Vec<String>) {
        self.websocket_servers = websocket_servers;
    }

    pub(crate) fn views_console_arguments(&self) -> &[ConsoleArgument] {
        &self.views_console_arguments
    }

    pub(crate) fn websocket_server_arguments(&self) -> &BTreeMap<String, Vec<ConsoleArgument>> {
        &self.websocket_server_arguments
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
        let mut context = BuildContext::new(&index, None);
        context.extend_modules(vec![GeneratedModuleTokens::new("broken", quote! { fn })]);

        let error = context
            .into_generated_code()
            .expect_err("an invalid generated module is rejected");

        assert!(
            error
                .to_string()
                .contains("failed to format a generated module")
        );
    }
}
