use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::polling_client_exports::PollingClientExports;
use crate::polling_client_module::PollingClientModule;

#[must_use]
pub fn render_polling_client_modules(
    module_name: &str,
    root_exports: &TokenStream,
    clients: &[PollingClientModule],
    PollingClientExports { client, verifier }: &PollingClientExports,
) -> Vec<GeneratedModuleTokens> {
    let mut submodule_declarations = Vec::new();
    let mut submodules = Vec::new();

    for polling_client in clients {
        if !polling_client.has_client && !polling_client.has_verifier {
            continue;
        }

        let segment = format_ident!("{}", polling_client.segment);
        let client_export = polling_client.has_client.then_some(client);
        let verifier_export = polling_client.has_verifier.then_some(verifier);

        submodule_declarations.push(quote! { pub mod #segment; });
        submodules.push(GeneratedModuleTokens::new(
            format!("{module_name}/{}", polling_client.segment),
            quote! { #client_export #verifier_export },
        ));
    }

    let mut modules = vec![GeneratedModuleTokens::new(
        module_name,
        quote! {
            #(#submodule_declarations)*
            #root_exports
        },
    )];

    modules.extend(submodules);

    modules
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_polling_client_modules;
    use crate::polling_client_exports::PollingClientExports;
    use crate::polling_client_module::PollingClientModule;

    fn exports() -> PollingClientExports {
        PollingClientExports {
            client: quote! { pub use framework::Client; },
            verifier: quote! { pub use framework::Verifier; },
        }
    }

    fn client(segment: &str, has_client: bool, has_verifier: bool) -> PollingClientModule {
        PollingClientModule {
            has_client,
            has_verifier,
            segment: segment.to_string(),
        }
    }

    fn render(clients: &[PollingClientModule]) -> Vec<GeneratedModuleTokens> {
        render_polling_client_modules("clients", &TokenStream::new(), clients, &exports())
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
    fn re_exports_a_per_tag_client_and_verifier_in_a_submodule() {
        let clients = [client("auth", true, true)];
        let root = module_source(render(&clients), "clients");
        let submodule = module_source(render(&clients), "clients/auth");

        assert!(root.contains("pub mod auth;"));
        assert!(submodule.contains("pub use framework::Client;"));
        assert!(submodule.contains("pub use framework::Verifier;"));
    }

    #[test]
    fn keeps_the_root_exports_beside_the_submodule_declarations() {
        let source = module_source(
            render_polling_client_modules(
                "clients",
                &quote! { pub use framework::Server; },
                &[],
                &exports(),
            ),
            "clients",
        );

        assert!(source.contains("pub use framework::Server;"));
    }

    #[test]
    fn renders_two_independent_clients_in_separate_submodules() {
        assert_eq!(
            module_names(&render(&[
                client("auth", true, true),
                client("partner", true, false)
            ])),
            vec![
                "clients".to_string(),
                "clients/auth".to_string(),
                "clients/partner".to_string(),
            ]
        );
    }

    #[test]
    fn omits_the_verifier_when_only_the_client_is_used() {
        let submodule = module_source(render(&[client("auth", true, false)]), "clients/auth");

        assert!(submodule.contains("pub use framework::Client;"));
        assert!(!submodule.contains("Verifier"));
    }

    #[test]
    fn omits_a_client_submodule_that_provides_nothing() {
        let modules = render(&[client("auth", false, false)]);

        assert_eq!(module_names(&modules), vec!["clients".to_string()]);
        assert!(!module_source(modules, "clients").contains("pub mod auth;"));
    }
}
