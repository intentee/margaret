use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn container_module_source(modules: Vec<GeneratedModuleTokens>) -> String {
    modules
        .into_iter()
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
