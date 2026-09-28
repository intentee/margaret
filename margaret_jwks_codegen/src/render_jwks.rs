use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_polling_client_codegen::polling_client_module::PollingClientModule;
use margaret_polling_client_codegen::render_polling_client_modules::render_polling_client_modules;

use crate::jwks_client_module::JwksClientModule;
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

fn client_module(
    JwksClientModule {
        has_client,
        has_verifier,
        segment,
    }: &JwksClientModule,
) -> Option<PollingClientModule> {
    let client = has_client.then(|| {
        quote! {
            pub use margaret::framework::jwks_client::jwks_client::JwksClient;
        }
    });
    let verifier = has_verifier.then(|| {
        quote! {
            pub use margaret::framework::jwks_client::public_jwks_verifier::PublicJwksVerifier;
        }
    });

    (*has_client || *has_verifier).then(|| PollingClientModule {
        exports: quote! { #client #verifier },
        segment: segment.clone(),
    })
}

#[must_use]
pub fn render_jwks(
    server: &JwksServerModule,
    clients: &[JwksClientModule],
) -> Vec<GeneratedModuleTokens> {
    render_polling_client_modules(
        JWKS_MODULE_NAME,
        &server_root_tokens(server),
        clients.iter().filter_map(client_module).collect(),
    )
}

#[cfg(test)]
mod tests {
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_jwks;
    use crate::jwks_client_module::JwksClientModule;
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

    fn client(segment: &str, has_client: bool, has_verifier: bool) -> JwksClientModule {
        JwksClientModule {
            has_client,
            has_verifier,
            segment: segment.to_string(),
        }
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
    fn re_exports_a_per_tag_client_and_verifier_in_a_submodule() {
        let modules = render_jwks(&no_server(), &[client("auth", true, true)]);
        let root = module_source(
            render_jwks(&no_server(), &[client("auth", true, true)]),
            "jwks",
        );
        let submodule = module_source(modules, "jwks/auth");

        assert!(root.contains("pub mod auth;"));
        assert!(
            submodule
                .contains("pub use margaret::framework::jwks_client::jwks_client::JwksClient;")
        );
        assert!(submodule.contains(
            "pub use margaret::framework::jwks_client::public_jwks_verifier::PublicJwksVerifier;"
        ));
    }

    #[test]
    fn omits_the_verifier_when_only_the_client_is_used() {
        let submodule = module_source(
            render_jwks(&no_server(), &[client("auth", true, false)]),
            "jwks/auth",
        );

        assert!(
            submodule
                .contains("pub use margaret::framework::jwks_client::jwks_client::JwksClient;")
        );
        assert!(!submodule.contains("PublicJwksVerifier"));
    }

    #[test]
    fn omits_a_client_submodule_that_provides_nothing() {
        let modules = render_jwks(&no_server(), &[client("auth", false, false)]);

        assert!(modules.iter().all(|module| module.name() != "jwks/auth"));
        assert!(!module_source(modules, "jwks").contains("pub mod auth;"));
    }
}
