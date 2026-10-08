use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_service_codegen::framework_service::FrameworkService;

pub(crate) struct FrameworkModules {
    pub(crate) modules: Vec<GeneratedModuleTokens>,
    pub(crate) services: Vec<FrameworkService>,
}
