use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;

pub(crate) struct WrapperModules<'tags> {
    pub(crate) middleware_plans: MiddlewarePlans<'tags>,
    pub(crate) modules: Vec<GeneratedModuleTokens>,
    pub(crate) registries: BindingRegistries,
}
