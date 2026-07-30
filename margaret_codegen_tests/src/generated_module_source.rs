use margaret_codegen::generated_code::GeneratedCode;
use margaret_generated_module::generated_module::GeneratedModule;

#[must_use]
pub fn generated_module_source<'code>(
    code: &'code GeneratedCode,
    name: &str,
) -> Option<&'code str> {
    code.modules()
        .iter()
        .find(|module| module.name() == name)
        .map(GeneratedModule::source)
}
