use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_polling_client_codegen::polling_client_module::PollingClientModule;
use margaret_polling_client_codegen::render_polling_client_modules::render_polling_client_modules;

use crate::jwks_module_name::JWKS_MODULE_NAME;
use crate::jwks_server_module::JwksServerModule;
use crate::jwks_server_part::JwksServerPart;

fn server_root_tokens(server: &JwksServerModule) -> TokenStream {
    let roller = server.contains(JwksServerPart::Roller).then(|| {
        quote! {
            pub use margaret::framework::jwks_roller_server::jwks_roller::JwksRoller;
        }
    });
    let handler = server.contains(JwksServerPart::Handler).then(|| {
        quote! {
            pub use margaret::framework::jwks_roller_server::public_jwks_handler::PublicJwksHandler;
        }
    });
    let secret_store = server.contains(JwksServerPart::SecretStore).then(|| {
        quote! {
            pub use margaret::framework::jwks_secret_store::jwks_secret_store::JwksSecretStore;
        }
    });
    let minter = server.contains(JwksServerPart::Minter).then(|| {
        quote! {
            pub use margaret::framework::access_token_minter::mint_access_token_handler::MintAccessTokenHandler;
        }
    });

    quote! { #roller #handler #secret_store #minter }
}

#[must_use]
pub fn render_jwks(server: &JwksServerModule, segments: &[&str]) -> Vec<GeneratedModuleTokens> {
    render_polling_client_modules(
        JWKS_MODULE_NAME,
        &server_root_tokens(server),
        segments
            .iter()
            .map(|segment| PollingClientModule {
                exports: quote! {
                    pub use margaret::framework::jwks_client::jwks_client::JwksClient;
                },
                segment: (*segment).to_string(),
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_jwks;
    use crate::jwks_server_module::JwksServerModule;
    use crate::jwks_server_part::JwksServerPart;

    fn no_server() -> JwksServerModule {
        JwksServerModule::default()
    }

    fn full_server() -> JwksServerModule {
        let mut server = JwksServerModule::default();

        for part in [
            JwksServerPart::Handler,
            JwksServerPart::Minter,
            JwksServerPart::Roller,
            JwksServerPart::SecretStore,
        ] {
            server.enable_if(part, true);
        }

        server
    }

    fn module_source(modules: Vec<GeneratedModuleTokens>, name: &str) -> String {
        modules
            .into_iter()
            .find(|module| module.name() == name)
            .expect("the module is generated")
            .format()
            .expect("the module formats")
            .source()
            .to_string()
    }

    #[test]
    fn re_exports_the_server_side_types_when_present() {
        let server = full_server();
        let source = module_source(render_jwks(&server, &[]), "jwks");

        assert!(
            source.contains(
                "pub use margaret::framework::jwks_roller_server::jwks_roller::JwksRoller;"
            )
        );
        assert!(source.contains(
            "pub use margaret::framework::jwks_roller_server::public_jwks_handler::PublicJwksHandler;"
        ));
        assert!(source.contains(
            "pub use margaret::framework::jwks_secret_store::jwks_secret_store::JwksSecretStore;"
        ));
        assert!(source.contains(
            "pub use margaret::framework::access_token_minter::mint_access_token_handler::MintAccessTokenHandler;"
        ));
    }

    #[test]
    fn omits_server_side_types_when_absent() {
        let source = module_source(render_jwks(&no_server(), &[]), "jwks");

        assert!(!source.contains("JwksRoller"));
        assert!(!source.contains("PublicJwksHandler"));
        assert!(!source.contains("JwksSecretStore"));
        assert!(!source.contains("MintAccessTokenHandler"));
    }

    #[test]
    fn re_exports_a_per_tag_client_in_a_submodule() {
        let modules = render_jwks(&no_server(), &["auth"]);
        let root = module_source(render_jwks(&no_server(), &["auth"]), "jwks");
        let submodule = module_source(modules, "jwks/auth");

        assert!(root.contains("pub mod auth;"));
        assert_eq!(
            submodule.trim(),
            "pub use margaret::framework::jwks_client::jwks_client::JwksClient;"
        );
    }
}
