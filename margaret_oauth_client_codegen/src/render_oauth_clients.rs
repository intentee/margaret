use quote::format_ident;
use quote::quote;
use syn::ext::IdentExt;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::client_id_module_name::CLIENT_ID_MODULE_NAME;
use crate::declared_oauth_clients::DeclaredOAuthClients;
use crate::oauth_client_declaration::OAuthClientDeclaration;
use crate::oauth_client_item::OAuthClientItem;
use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;

fn client_modules(
    OAuthClientDeclaration { client_id, tag, .. }: &OAuthClientDeclaration,
) -> [GeneratedModuleTokens; 2] {
    let client_module = format!("{OAUTH_CLIENTS_MODULE_NAME}/{}", tag.ident().unraw());
    let client_id_module = format_ident!("{CLIENT_ID_MODULE_NAME}");
    let client_id = client_id.as_str();
    let exports = OAuthClientItem::ALL.map(|item| {
        let framework_path = item.framework_path();

        quote! { pub use #framework_path; }
    });

    [
        GeneratedModuleTokens::new(
            format!("{client_module}/{CLIENT_ID_MODULE_NAME}"),
            quote! { pub const CLIENT_ID: &str = #client_id; },
        ),
        GeneratedModuleTokens::new(
            client_module,
            quote! { pub mod #client_id_module; #(#exports)* },
        ),
    ]
}

#[must_use]
pub fn render_oauth_clients(clients: &DeclaredOAuthClients) -> Vec<GeneratedModuleTokens> {
    let submodules = clients.clients.iter().map(|client| {
        let module = client.tag.ident();

        quote! { pub mod #module; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        OAUTH_CLIENTS_MODULE_NAME,
        quote! { #(#submodules)* },
    )];

    modules.extend(clients.clients.iter().flat_map(client_modules));

    modules
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_oauth_clients;
    use crate::declared_oauth_clients::DeclaredOAuthClients;

    const TWO_CLIENTS: &str = "#[oauth_client(ci, authentication = private_key_jwt, client_id = \"ci\", issuer = partner)]\npub struct Ci;\n#[oauth_client(billing, authentication = client_secret_basic(client_secret_from = \"BILLING_SECRET\"), client_id = \"billing:app\", issuer = partner)]\npub struct Billing;\n";

    fn modules(lib_source: &str) -> Vec<GeneratedModuleTokens> {
        let indexed = IndexedSource::new(lib_source);

        render_oauth_clients(
            &DeclaredOAuthClients::read(&indexed.index).expect("the clients are read"),
        )
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
    fn declares_a_submodule_per_oauth_client_in_tag_order() {
        assert_eq!(
            module_source(modules(TWO_CLIENTS), "oauth_clients"),
            "pub mod billing;\npub mod ci;\n"
        );
    }

    #[test]
    fn re_exports_the_client_runtime_beside_its_client_id() {
        assert_eq!(
            module_source(modules(TWO_CLIENTS), "oauth_clients/billing"),
            "pub mod client_id;\npub use margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient;\npub use margaret::framework::client_credentials::client_credentials::ClientCredentials;\npub use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;\npub use margaret::framework::token_exchange_client::token_exchange::TokenExchange;\n"
        );
    }

    #[test]
    fn renders_the_declared_client_id() {
        assert_eq!(
            module_source(modules(TWO_CLIENTS), "oauth_clients/billing/client_id"),
            "pub const CLIENT_ID: &str = \"billing:app\";\n"
        );
    }

    #[test]
    fn names_the_module_of_a_raw_identifier_tag_after_its_identifier() {
        let modules = modules(
            "#[oauth_client(r#async, authentication = private_key_jwt, client_id = \"async\", issuer = partner)]\npub struct Async;\n",
        );

        assert!(
            modules
                .iter()
                .any(|module| module.name() == "oauth_clients/async/client_id")
        );
        assert_eq!(
            module_source(modules, "oauth_clients"),
            "pub mod r#async;\n"
        );
    }
}
