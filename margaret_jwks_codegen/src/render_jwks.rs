use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::jwks_client_module::JwksClientModule;
use crate::jwks_module_name::JWKS_MODULE_NAME;
use crate::jwks_server_module::JwksServerModule;

fn client_submodule_tokens(client: &JwksClientModule) -> TokenStream {
    let client_use = client.has_client.then(|| {
        quote! {
            pub use margaret_jwks_client::jwks_client::JwksClient;
        }
    });
    let verifier_use = client.has_verifier.then(|| {
        quote! {
            pub use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
        }
    });

    quote! { #client_use #verifier_use }
}

fn server_root_tokens(server: &JwksServerModule) -> TokenStream {
    let roller = server.has_roller.then(|| {
        quote! {
            pub use margaret_jwks_roller_server::jwks_roller::JwksRoller;
        }
    });
    let handler = server.has_handler.then(|| {
        quote! {
            pub use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;
        }
    });
    let secret_store = server.has_secret_store.then(|| {
        quote! {
            pub use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
        }
    });
    let minter = server.has_minter.then(|| {
        quote! {
            pub use margaret_access_token_minter::mint_access_token_handler::MintAccessTokenHandler;
        }
    });

    quote! { #roller #handler #secret_store #minter }
}

#[must_use]
pub fn render_jwks(
    server: &JwksServerModule,
    clients: &[JwksClientModule],
) -> Vec<GeneratedModuleTokens> {
    let mut submodule_declarations = Vec::new();
    let mut submodules = Vec::new();

    for client in clients {
        if !client.has_client && !client.has_verifier {
            continue;
        }

        let segment = format_ident!("{}", client.segment);

        submodule_declarations.push(quote! { pub mod #segment; });
        submodules.push(GeneratedModuleTokens::new(
            format!("{JWKS_MODULE_NAME}/{}", client.segment),
            client_submodule_tokens(client),
        ));
    }

    let server_root = server_root_tokens(server);
    let mut modules = vec![GeneratedModuleTokens::new(
        JWKS_MODULE_NAME,
        quote! {
            #server_root
            #(#submodule_declarations)*
        },
    )];

    modules.extend(submodules);

    modules
}

#[cfg(test)]
mod tests {
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_jwks;
    use crate::jwks_client_module::JwksClientModule;
    use crate::jwks_server_module::JwksServerModule;

    fn no_server() -> JwksServerModule {
        JwksServerModule {
            has_handler: false,
            has_minter: false,
            has_roller: false,
            has_secret_store: false,
        }
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

    fn module_names(modules: &[GeneratedModuleTokens]) -> Vec<String> {
        modules
            .iter()
            .map(|module| module.name().to_string())
            .collect()
    }

    #[test]
    fn re_exports_the_server_side_types_when_present() {
        let server = JwksServerModule {
            has_handler: true,
            has_minter: true,
            has_roller: true,
            has_secret_store: true,
        };
        let source = module_source(render_jwks(&server, &[]), "jwks");

        assert!(source.contains("pub use margaret_jwks_roller_server::jwks_roller::JwksRoller;"));
        assert!(source.contains(
            "pub use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;"
        ));
        assert!(
            source.contains("pub use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;")
        );
        assert!(source.contains(
            "pub use margaret_access_token_minter::mint_access_token_handler::MintAccessTokenHandler;"
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
        assert!(submodule.contains("pub use margaret_jwks_client::jwks_client::JwksClient;"));
        assert!(submodule.contains(
            "pub use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;"
        ));
    }

    #[test]
    fn renders_two_independent_clients_in_separate_submodules() {
        let modules = render_jwks(
            &no_server(),
            &[client("auth", true, true), client("partner", true, false)],
        );

        assert_eq!(
            module_names(&modules),
            vec![
                "jwks".to_string(),
                "jwks/auth".to_string(),
                "jwks/partner".to_string(),
            ]
        );
    }

    #[test]
    fn omits_the_verifier_when_only_the_client_is_used() {
        let submodule = module_source(
            render_jwks(&no_server(), &[client("auth", true, false)]),
            "jwks/auth",
        );

        assert!(submodule.contains("pub use margaret_jwks_client::jwks_client::JwksClient;"));
        assert!(!submodule.contains("PublicJwksVerifier"));
    }

    #[test]
    fn omits_a_client_submodule_that_provides_nothing() {
        let modules = render_jwks(&no_server(), &[client("auth", false, false)]);

        assert_eq!(module_names(&modules), vec!["jwks".to_string()]);
        assert!(!module_source(modules, "jwks").contains("pub mod auth;"));
    }
}
