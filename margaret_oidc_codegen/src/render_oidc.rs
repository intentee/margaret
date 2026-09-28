use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_polling_client_codegen::polling_client_exports::PollingClientExports;
use margaret_polling_client_codegen::polling_client_module::PollingClientModule;
use margaret_polling_client_codegen::render_polling_client_modules::render_polling_client_modules;

use crate::oidc_module_name::OIDC_MODULE_NAME;

#[must_use]
pub fn render_oidc(clients: &[PollingClientModule]) -> Vec<GeneratedModuleTokens> {
    render_polling_client_modules(
        OIDC_MODULE_NAME,
        &TokenStream::new(),
        clients,
        &PollingClientExports {
            client: quote! {
                pub use margaret::framework::oidc_client::oidc_client::OidcClient;
            },
            verifier: quote! {
                pub use margaret::framework::oidc_client::oidc_token_verifier::OidcTokenVerifier;
            },
        },
    )
}

#[cfg(test)]
mod tests {
    use margaret_polling_client_codegen::polling_client_module::PollingClientModule;

    use super::render_oidc;

    #[test]
    fn re_exports_the_client_and_verifier_of_an_issuer_in_its_segment() {
        let source = render_oidc(&[PollingClientModule {
            has_client: true,
            has_verifier: true,
            segment: "github_issuer".to_string(),
        }])
        .into_iter()
        .find(|module| module.name() == "oidc/github_issuer")
        .expect("the issuer segment is generated")
        .format()
        .expect("the module formats")
        .source()
        .to_string();

        assert!(
            source.contains("pub use margaret::framework::oidc_client::oidc_client::OidcClient;")
        );
        assert!(source.contains(
            "pub use margaret::framework::oidc_client::oidc_token_verifier::OidcTokenVerifier;"
        ));
    }
}
