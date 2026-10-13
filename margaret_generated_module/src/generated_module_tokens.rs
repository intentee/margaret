use proc_macro2::TokenStream;
use syn::File;

use crate::generated_module::GeneratedModule;
use crate::generated_module_error::GeneratedModuleError;
use crate::module_path::ModulePath;

#[derive(Debug)]
pub struct GeneratedModuleTokens {
    path: ModulePath,
    tokens: TokenStream,
}

impl GeneratedModuleTokens {
    #[must_use]
    pub fn new(path: ModulePath, tokens: TokenStream) -> Self {
        Self { path, tokens }
    }

    /// # Errors
    ///
    /// Returns `GeneratedModuleError::InvalidGeneratedFile`.
    pub fn format(self) -> Result<GeneratedModule, GeneratedModuleError> {
        match syn::parse2::<File>(self.tokens) {
            Ok(file) => Ok(GeneratedModule::new(
                self.path,
                prettyplease::unparse(&file),
            )),
            Err(source) => Err(GeneratedModuleError::InvalidGeneratedFile {
                path: self.path,
                source,
            }),
        }
    }

    #[must_use]
    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    #[must_use]
    pub fn to_source(&self) -> String {
        self.tokens.to_string()
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::GeneratedModuleTokens;
    use crate::module_path::ModulePath;

    #[test]
    fn formats_tokens_into_pretty_source_so_generated_files_stay_human_readable() {
        let generated = GeneratedModuleTokens::new(
            ModulePath::root("sample"),
            quote! { pub fn answer() -> u8 { 42 } },
        )
        .format()
        .expect("the tokens form a valid file");

        assert_eq!(generated.path().to_string(), "sample");
        assert_eq!(generated.source(), "pub fn answer() -> u8 {\n    42\n}\n");
    }

    #[test]
    fn reports_tokens_that_are_not_a_valid_rust_file() {
        let error = GeneratedModuleTokens::new(ModulePath::root("broken"), quote! { fn })
            .format()
            .expect_err("invalid tokens are rejected");

        assert!(error.to_string().contains("do not form a valid Rust file"));
    }

    #[test]
    fn exposes_the_unformatted_token_source_while_keeping_the_tokens_for_formatting() {
        let module = GeneratedModuleTokens::new(
            ModulePath::root("sample"),
            quote! { pub fn answer() -> u8 { 42 } },
        );

        assert_eq!(module.path().to_string(), "sample");
        assert_eq!(module.to_source(), "pub fn answer () -> u8 { 42 }");
    }
}
