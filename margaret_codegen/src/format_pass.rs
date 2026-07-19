use margaret_generated_module::generated_module::GeneratedModule;
use margaret_generated_module::generated_module_error::GeneratedModuleError;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

pub(crate) fn format_pass(
    modules: Vec<GeneratedModuleTokens>,
) -> Result<Vec<GeneratedModule>, GeneratedModuleError> {
    modules
        .into_iter()
        .map(GeneratedModuleTokens::format)
        .collect()
}
