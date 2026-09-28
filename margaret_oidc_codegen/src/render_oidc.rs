use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_polling_client_codegen::polling_client_module::PollingClientModule;
use margaret_polling_client_codegen::render_polling_client_modules::render_polling_client_modules;

use crate::oidc_module_name::OIDC_MODULE_NAME;

#[must_use]
pub fn render_oidc(segments: &[&str]) -> Vec<GeneratedModuleTokens> {
    render_polling_client_modules(
        OIDC_MODULE_NAME,
        &TokenStream::new(),
        segments
            .iter()
            .map(|segment| PollingClientModule {
                exports: quote! {
                    pub use margaret::framework::oidc_client::oidc_client::OidcClient;
                },
                segment: (*segment).to_string(),
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::render_oidc;

    #[test]
    fn re_exports_the_client_of_an_issuer_in_its_segment() {
        let source = render_oidc(&["github_issuer"])
            .into_iter()
            .find(|module| module.name() == "oidc/github_issuer")
            .expect("the issuer segment is generated")
            .format()
            .expect("the module formats")
            .source()
            .to_string();

        assert_eq!(
            source.trim(),
            "pub use margaret::framework::oidc_client::oidc_client::OidcClient;"
        );
    }
}
