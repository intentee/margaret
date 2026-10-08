use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ext::IdentExt;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_vocabulary::scope::Scope;

use crate::client_id_module_name::CLIENT_ID_MODULE_NAME;
use crate::module_sign_in::ModuleSignIn;
use crate::oauth_client_item::OAuthClientItem;
use crate::oauth_client_module::OAuthClientModule;
use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;
use crate::sign_in_scopes_module_name::SIGN_IN_SCOPES_MODULE_NAME;

fn sign_in_scopes_modules(
    client_module: &str,
    sign_in: &ModuleSignIn,
) -> Vec<GeneratedModuleTokens> {
    match sign_in {
        ModuleSignIn::Available { scopes } => {
            let scopes = scopes.iter().map(Scope::as_str);

            vec![GeneratedModuleTokens::new(
                format!("{client_module}/{SIGN_IN_SCOPES_MODULE_NAME}"),
                quote! { pub const SIGN_IN_SCOPES: &[&str] = &[#(#scopes),*]; },
            )]
        }
        ModuleSignIn::Unavailable => Vec::new(),
    }
}

fn sign_in_scopes_declaration(sign_in: &ModuleSignIn) -> TokenStream {
    match sign_in {
        ModuleSignIn::Available { .. } => {
            let module = format_ident!("{SIGN_IN_SCOPES_MODULE_NAME}");

            quote! { pub mod #module; }
        }
        ModuleSignIn::Unavailable => TokenStream::new(),
    }
}

fn client_modules(
    OAuthClientModule {
        client_id,
        sign_in,
        tag,
    }: &OAuthClientModule,
) -> Vec<GeneratedModuleTokens> {
    let client_module = format!("{OAUTH_CLIENTS_MODULE_NAME}/{}", tag.ident().unraw());
    let client_id_module = format_ident!("{CLIENT_ID_MODULE_NAME}");
    let client_id = client_id.as_str();
    let sign_in_scopes = sign_in_scopes_declaration(sign_in);
    let exports = OAuthClientItem::ALL
        .into_iter()
        .filter(|item| item.is_available_to(sign_in))
        .map(|item| {
            let framework_path = item.framework_path();

            quote! { pub use #framework_path; }
        });

    [
        GeneratedModuleTokens::new(
            format!("{client_module}/{CLIENT_ID_MODULE_NAME}"),
            quote! { pub const CLIENT_ID: &str = #client_id; },
        ),
        GeneratedModuleTokens::new(
            client_module.clone(),
            quote! { pub mod #client_id_module; #sign_in_scopes #(#exports)* },
        ),
    ]
    .into_iter()
    .chain(sign_in_scopes_modules(&client_module, sign_in))
    .collect()
}

#[must_use]
pub fn render_oauth_clients(clients: &[OAuthClientModule]) -> Vec<GeneratedModuleTokens> {
    let submodules = clients.iter().map(|client| {
        let module = client.tag.ident();

        quote! { pub mod #module; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        OAUTH_CLIENTS_MODULE_NAME,
        quote! { #(#submodules)* },
    )];

    modules.extend(clients.iter().flat_map(client_modules));

    modules
}

#[cfg(test)]
mod tests {
    use margaret_attributes::tag::Tag;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_oauth_vocabulary::client_id::ClientId;
    use margaret_oauth_vocabulary::scope::Scope;
    use syn::parse_str;

    use super::render_oauth_clients;
    use crate::module_sign_in::ModuleSignIn;
    use crate::oauth_client_module::OAuthClientModule;

    fn client_id(client_id: &str) -> ClientId {
        serde_json::from_value(serde_json::Value::String(client_id.to_string()))
            .expect("the client id is valid")
    }

    fn tag(tag: &str) -> Tag {
        Tag::from_path(&parse_str(tag).expect("the tag is a path")).expect("the tag is plain")
    }

    fn two_clients() -> Vec<GeneratedModuleTokens> {
        let billing_id = client_id("billing:app");
        let billing = tag("billing");
        let ci_id = client_id("ci");
        let ci = tag("ci");
        let scopes: Vec<Scope> = ["openid", "profile"]
            .iter()
            .map(|scope| {
                serde_json::from_value(serde_json::Value::String((*scope).to_string()))
                    .expect("the scope is valid")
            })
            .collect();

        render_oauth_clients(&[
            OAuthClientModule {
                client_id: &billing_id,
                sign_in: ModuleSignIn::Unavailable,
                tag: &billing,
            },
            OAuthClientModule {
                client_id: &ci_id,
                sign_in: ModuleSignIn::Available { scopes: &scopes },
                tag: &ci,
            },
        ])
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
    fn declares_a_submodule_per_oauth_client() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients"),
            "pub mod billing;\npub mod ci;\n"
        );
    }

    #[test]
    fn re_exports_the_client_runtime_beside_its_client_id() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients/billing"),
            "pub mod client_id;\npub use margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient;\npub use margaret::framework::client_credentials::client_credentials::ClientCredentials;\npub use margaret::framework::token_exchange_client::token_exchange::TokenExchange;\n"
        );
    }

    #[test]
    fn re_exports_the_sign_in_flow_beside_the_scopes_of_a_client_that_signs_in() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients/ci"),
            "pub mod client_id;\npub mod sign_in_scopes;\npub use margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient;\npub use margaret::framework::client_credentials::client_credentials::ClientCredentials;\npub use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;\npub use margaret::framework::token_exchange_client::token_exchange::TokenExchange;\n"
        );
    }

    #[test]
    fn renders_the_declared_client_id() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients/billing/client_id"),
            "pub const CLIENT_ID: &str = \"billing:app\";\n"
        );
    }

    #[test]
    fn renders_the_scopes_a_client_signs_in_with() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients/ci/sign_in_scopes"),
            "pub const SIGN_IN_SCOPES: &[&str] = &[\"openid\", \"profile\"];\n"
        );
    }

    #[test]
    fn names_the_module_of_a_raw_identifier_tag_after_its_identifier() {
        let async_id = client_id("async");
        let raw = tag("r#async");
        let modules = render_oauth_clients(&[OAuthClientModule {
            client_id: &async_id,
            sign_in: ModuleSignIn::Unavailable,
            tag: &raw,
        }]);

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
