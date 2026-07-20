use proc_macro2::TokenStream;

use crate::generated_module::GeneratedModule;
use crate::generated_module_error::GeneratedModuleError;

#[derive(Debug)]
pub struct GeneratedModuleTokens {
    name: String,
    tokens: TokenStream,
}

impl GeneratedModuleTokens {
    pub fn new(name: impl Into<String>, tokens: TokenStream) -> Self {
        Self {
            name: name.into(),
            tokens,
        }
    }

    pub fn format(self) -> Result<GeneratedModule, GeneratedModuleError> {
        let file = syn::parse2::<syn::File>(self.tokens).map_err(|source| {
            GeneratedModuleError::InvalidGeneratedFile {
                name: self.name.clone(),
                source,
            }
        })?;

        Ok(GeneratedModule::new(
            self.name,
            prettyplease::unparse(&file),
        ))
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
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

    #[test]
    fn formats_tokens_into_pretty_source() {
        let generated =
            GeneratedModuleTokens::new("sample", quote! { pub fn answer() -> u8 { 42 } })
                .format()
                .expect("the tokens form a valid file");

        assert_eq!(generated.name(), "sample");
        assert_eq!(generated.source(), "pub fn answer() -> u8 {\n    42\n}\n");
    }

    #[test]
    fn reports_tokens_that_are_not_a_valid_rust_file() {
        let error = GeneratedModuleTokens::new("broken", quote! { fn })
            .format()
            .expect_err("invalid tokens are rejected");

        assert!(error.to_string().contains("do not form a valid Rust file"));
    }

    #[test]
    fn exposes_the_unformatted_token_source() {
        let module = GeneratedModuleTokens::new("sample", quote! { pub fn answer() -> u8 { 42 } });

        assert_eq!(module.name(), "sample");
        assert_eq!(module.to_source(), "pub fn answer () -> u8 { 42 }");
    }
}
