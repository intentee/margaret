use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::polling_client_module::PollingClientModule;

#[must_use]
pub fn render_polling_client_modules(
    module_name: &str,
    root_exports: &TokenStream,
    clients: Vec<PollingClientModule>,
) -> Vec<GeneratedModuleTokens> {
    let submodule_declarations = clients.iter().map(|polling_client| {
        let segment = format_ident!("{}", polling_client.segment);

        quote! { pub mod #segment; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        module_name,
        quote! {
            #(#submodule_declarations)*
            #root_exports
        },
    )];

    modules.extend(
        clients
            .into_iter()
            .map(|PollingClientModule { exports, segment }| {
                GeneratedModuleTokens::new(format!("{module_name}/{segment}"), exports)
            }),
    );

    modules
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_polling_client_modules;
    use crate::polling_client_module::PollingClientModule;

    fn client(segment: &str) -> PollingClientModule {
        PollingClientModule {
            exports: quote! { pub use framework::Client; },
            segment: segment.to_string(),
        }
    }

    fn render(clients: Vec<PollingClientModule>) -> Vec<GeneratedModuleTokens> {
        render_polling_client_modules("clients", &TokenStream::new(), clients)
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
    fn re_exports_the_exports_of_a_client_in_its_submodule() {
        let root = module_source(render(vec![client("auth")]), "clients");
        let submodule = module_source(render(vec![client("auth")]), "clients/auth");

        assert!(root.contains("pub mod auth;"));
        assert!(submodule.contains("pub use framework::Client;"));
    }

    #[test]
    fn keeps_the_root_exports_beside_the_submodule_declarations() {
        let source = module_source(
            render_polling_client_modules(
                "clients",
                &quote! { pub use framework::Server; },
                Vec::new(),
            ),
            "clients",
        );

        assert!(source.contains("pub use framework::Server;"));
    }

    #[test]
    fn renders_two_independent_clients_in_separate_submodules() {
        assert_eq!(
            module_names(&render(vec![client("auth"), client("partner")])),
            vec![
                "clients".to_string(),
                "clients/auth".to_string(),
                "clients/partner".to_string(),
            ]
        );
    }
}
