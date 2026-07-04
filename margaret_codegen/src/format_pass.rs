use margaret_generated_module::generated_module::GeneratedModule;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

pub(crate) fn format_pass(modules: Vec<GeneratedModuleTokens>) -> Vec<GeneratedModule> {
    modules
        .into_iter()
        .map(GeneratedModuleTokens::format)
        .collect()
}
