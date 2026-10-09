use std::iter;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ext::IdentExt;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_vocabulary::resource_scope::ResourceScope;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_token_issuance_codegen::resource_audience_path::resource_audience_path;

use crate::client_id_module_name::CLIENT_ID_MODULE_NAME;
use crate::module_sign_in::ModuleSignIn;
use crate::oauth_client_credentials::OAuthClientCredentials;
use crate::oauth_client_item::OAuthClientItem;
use crate::oauth_client_module::OAuthClientModule;
use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;
use crate::resource_grant_module_name::RESOURCE_GRANT_MODULE_NAME;
use crate::resources_module_name::RESOURCES_MODULE_NAME;
use crate::sign_in_callback_handler_module_name::SIGN_IN_CALLBACK_HANDLER_MODULE_NAME;
use crate::sign_in_scopes_module_name::SIGN_IN_SCOPES_MODULE_NAME;

fn sign_in_modules(client_module: &str, sign_in: &ModuleSignIn) -> Vec<GeneratedModuleTokens> {
    match sign_in {
        ModuleSignIn::Available { admission, scopes } => {
            let admission = path_tokens(admission);
            let scopes = scopes.iter().map(Scope::as_str);

            vec![
                GeneratedModuleTokens::new(
                    format!("{client_module}/{SIGN_IN_CALLBACK_HANDLER_MODULE_NAME}"),
                    quote! {
                        pub type SignInCallbackHandler =
                            margaret::framework::oidc_sign_in::sign_in_callback_handler::SignInCallbackHandler<#admission>;
                    },
                ),
                GeneratedModuleTokens::new(
                    format!("{client_module}/{SIGN_IN_SCOPES_MODULE_NAME}"),
                    quote! { pub const SIGN_IN_SCOPES: &[&str] = &[#(#scopes),*]; },
                ),
            ]
        }
        ModuleSignIn::Unavailable => Vec::new(),
    }
}

fn sign_in_declarations(sign_in: &ModuleSignIn) -> TokenStream {
    match sign_in {
        ModuleSignIn::Available { .. } => {
            let callback_handler = format_ident!("{SIGN_IN_CALLBACK_HANDLER_MODULE_NAME}");
            let scopes = format_ident!("{SIGN_IN_SCOPES_MODULE_NAME}");

            quote! { pub mod #callback_handler; pub mod #scopes; }
        }
        ModuleSignIn::Unavailable => TokenStream::new(),
    }
}

fn resource_modules(
    client_module: &str,
    credentials: &OAuthClientCredentials,
) -> Vec<GeneratedModuleTokens> {
    let OAuthClientCredentials::PerResource { resources, scopes } = credentials else {
        return Vec::new();
    };
    let parent_module = format!("{client_module}/{RESOURCES_MODULE_NAME}");
    let submodules = resources.iter().map(|resource| {
        let module = resource.ident();

        quote! { pub mod #module; }
    });
    let scopes = scopes
        .iter()
        .map(ResourceScope::as_str)
        .collect::<Vec<&str>>();
    let grant_module = format_ident!("{RESOURCE_GRANT_MODULE_NAME}");

    iter::once(GeneratedModuleTokens::new(
        parent_module.clone(),
        quote! { #(#submodules)* },
    ))
    .chain(resources.iter().flat_map(|resource| {
        let resource_module = format!("{parent_module}/{}", resource.ident().unraw());
        let audience = path_tokens(&resource_audience_path(resource));

        [
            GeneratedModuleTokens::new(
                resource_module.clone(),
                quote! {
                    pub mod #grant_module;
                    pub use margaret::framework::client_credentials::resource_credentials::ResourceCredentials;
                },
            ),
            GeneratedModuleTokens::new(
                format!("{resource_module}/{RESOURCE_GRANT_MODULE_NAME}"),
                quote! {
                    pub const RESOURCE_GRANT: margaret::framework::client_credentials::resource_grant::ResourceGrant =
                        margaret::framework::client_credentials::resource_grant::ResourceGrant {
                            audience: #audience,
                            scopes: &[#(#scopes),*],
                        };
                },
            ),
        ]
    }))
    .collect()
}

fn resources_declaration(credentials: &OAuthClientCredentials) -> TokenStream {
    match credentials {
        OAuthClientCredentials::PerResource { .. } => {
            let module = format_ident!("{RESOURCES_MODULE_NAME}");

            quote! { pub mod #module; }
        }
        OAuthClientCredentials::Targeted | OAuthClientCredentials::Withheld => TokenStream::new(),
    }
}

fn client_modules(
    OAuthClientModule {
        client_id,
        credentials,
        sign_in,
        tag,
    }: &OAuthClientModule,
) -> Vec<GeneratedModuleTokens> {
    let client_module = format!("{OAUTH_CLIENTS_MODULE_NAME}/{}", tag.ident().unraw());
    let client_id_module = format_ident!("{CLIENT_ID_MODULE_NAME}");
    let client_id = client_id.as_str();
    let sign_in_declarations = sign_in_declarations(sign_in);
    let resources_declaration = resources_declaration(credentials);
    let exports = OAuthClientItem::ALL
        .into_iter()
        .filter(|item| item.is_available_to(sign_in, credentials))
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
            quote! { pub mod #client_id_module; #resources_declaration #sign_in_declarations #(#exports)* },
        ),
    ]
    .into_iter()
    .chain(resource_modules(&client_module, credentials))
    .chain(sign_in_modules(&client_module, sign_in))
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
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::tag::Tag;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_oauth_vocabulary::client_id::ClientId;
    use margaret_oauth_vocabulary::scope::Scope;
    use syn::parse_str;

    use super::render_oauth_clients;
    use crate::module_sign_in::ModuleSignIn;
    use crate::oauth_client_credentials::OAuthClientCredentials;
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
        let admission = CanonicalPath::new(vec!["crate".to_string(), "CiReaders".to_string()]);
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
                credentials: &OAuthClientCredentials::Targeted,
                sign_in: ModuleSignIn::Unavailable,
                tag: &billing,
            },
            OAuthClientModule {
                client_id: &ci_id,
                credentials: &OAuthClientCredentials::Withheld,
                sign_in: ModuleSignIn::Available {
                    admission: &admission,
                    scopes: &scopes,
                },
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
    fn re_exports_the_sign_in_runtime_beside_the_scopes_of_a_client_that_signs_in() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients/ci"),
            "pub mod client_id;\npub mod sign_in_callback_handler;\npub mod sign_in_scopes;\npub use margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient;\npub use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;\npub use margaret::framework::oidc_sign_in::sign_in_start_handler::SignInStartHandler;\npub use margaret::framework::token_exchange_client::token_exchange::TokenExchange;\n"
        );
    }

    #[test]
    fn renders_the_callback_handler_admitting_through_the_declared_admission() {
        assert_eq!(
            module_source(two_clients(), "oauth_clients/ci/sign_in_callback_handler"),
            "pub type SignInCallbackHandler = margaret::framework::oidc_sign_in::sign_in_callback_handler::SignInCallbackHandler<\n    crate::CiReaders,\n>;\n"
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
            credentials: &OAuthClientCredentials::Withheld,
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
