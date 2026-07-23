use margaret_codegen::generated_code::GeneratedCode;

#[must_use]
pub fn generated_source(code: &GeneratedCode) -> String {
    let mut source = String::new();

    for module in code.modules() {
        if !source.is_empty() {
            source.push('\n');
        }

        source.push_str(module.source());
    }

    source
}
