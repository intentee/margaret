use std::path::PathBuf;

use syn::parse_quote;

use crate::scaffolded_module::ScaffoldedModule;

pub(crate) fn render_service_build_script() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            fn main() -> Result<(), margaret::framework::codegen::codegen_error::CodegenError> {
                margaret::framework::codegen::generate::generate()
            }
        },
        relative_path: PathBuf::from("build.rs"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_service_build_script;
    use crate::format_scaffolded_module::format_scaffolded_module;

    #[test]
    fn runs_the_framework_codegen_for_the_scaffolded_crate() {
        let rendered = format_scaffolded_module(render_service_build_script());

        assert_eq!(rendered.relative_path, PathBuf::from("build.rs"));
        assert_eq!(
            rendered.contents,
            concat!(
                "fn main() -> Result<(), margaret::framework::codegen::codegen_error::CodegenError> {\n",
                "    margaret::framework::codegen::generate::generate()\n",
                "}\n",
            )
        );
    }
}
