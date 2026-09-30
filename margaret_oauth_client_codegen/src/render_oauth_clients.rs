use quote::quote;
use syn::ext::IdentExt;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;

use crate::oauth_client_item::OAuthClientItem;
use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;

#[must_use]
pub fn render_oauth_clients(bindings: &[OAuthClientBinding]) -> Vec<GeneratedModuleTokens> {
    let submodules = bindings.iter().map(|binding| {
        let module = binding.tag.ident();

        quote! { pub mod #module; }
    });
    let exports = OAuthClientItem::ALL.map(|item| {
        let framework_path = item.framework_path();

        quote! { pub use #framework_path; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        OAUTH_CLIENTS_MODULE_NAME,
        quote! { #(#submodules)* },
    )];

    modules.extend(bindings.iter().map(|binding| {
        GeneratedModuleTokens::new(
            format!(
                "{OAUTH_CLIENTS_MODULE_NAME}/{}",
                binding.tag.ident().unraw()
            ),
            quote! { #(#exports)* },
        )
    }));

    modules
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::tag::Tag;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
    use margaret_tag_codegen::trusted_issuer_binding::TrustedIssuerBinding;
    use margaret_tag_codegen::trusted_issuer_kind::TrustedIssuerKind;
    use syn::Path;

    use super::render_oauth_clients;

    fn tag(name: &str) -> Tag {
        let path: Path = syn::parse_str(name).expect("the tag path parses");

        Tag::from_path(&path).expect("the tag is a plain name")
    }

    fn issuer() -> TrustedIssuerBinding {
        TrustedIssuerBinding {
            declaring: CanonicalPath::new(vec!["crate".to_string(), "Issuer".to_string()]),
            kind: TrustedIssuerKind::OidcIssuer,
            module_segment: "issuer".to_string(),
            tag: tag("partner"),
        }
    }

    fn client<'bindings>(
        issuer: &'bindings TrustedIssuerBinding,
        client_tag: &str,
    ) -> OAuthClientBinding<'bindings> {
        OAuthClientBinding {
            declaring: CanonicalPath::new(vec!["crate".to_string(), "Client".to_string()]),
            issuer,
            tag: tag(client_tag),
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
    fn declares_a_submodule_per_oauth_client() {
        let issuer = issuer();

        assert_eq!(
            module_source(
                render_oauth_clients(&[client(&issuer, "billing"), client(&issuer, "ci")]),
                "oauth_clients",
            )
            .trim(),
            "pub mod billing;\npub mod ci;"
        );
    }

    #[test]
    fn names_the_module_of_a_raw_identifier_tag_after_its_identifier() {
        let issuer = issuer();
        let modules = render_oauth_clients(&[client(&issuer, "r#async")]);

        assert!(
            modules
                .iter()
                .any(|module| module.name() == "oauth_clients/async")
        );
        assert_eq!(
            module_source(modules, "oauth_clients").trim(),
            "pub mod r#async;"
        );
    }

    #[test]
    fn re_exports_the_client_runtime_in_its_submodule() {
        let issuer = issuer();

        assert_eq!(
            module_source(
                render_oauth_clients(&[client(&issuer, "billing")]),
                "oauth_clients/billing",
            )
            .trim(),
            "pub use margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient;\npub use margaret::framework::client_credentials::client_credentials::ClientCredentials;\npub use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;\npub use margaret::framework::token_exchange_client::token_exchange::TokenExchange;"
        );
    }
}
