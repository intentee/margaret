use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ext::IdentExt;

use margaret_attributes::tag::Tag;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_umbrella_path::routes_path::routes_path;

use crate::authorization_handler_module_name::AUTHORIZATION_HANDLER_MODULE_NAME;
use crate::consent_page::ConsentPage;
use crate::derived_authorization::DerivedAuthorization;
use crate::oidc_provider_item::OidcProviderItem;
use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoints_module_name::PROVIDER_ENDPOINTS_MODULE_NAME;
use crate::served_authorization::ServedAuthorization;
use crate::subject_token_exchangers_module_name::SUBJECT_TOKEN_EXCHANGERS_MODULE_NAME;

fn authorization_handler_module(
    ServedAuthorization {
        consent: ConsentPage { view, .. },
        ..
    }: &ServedAuthorization,
) -> GeneratedModuleTokens {
    let view = path_tokens(view);
    let routes = path_tokens(&routes_path());

    GeneratedModuleTokens::new(
        format!("{OIDC_PROVIDER_MODULE_NAME}/{AUTHORIZATION_HANDLER_MODULE_NAME}"),
        quote! {
            pub type AuthorizationHandler =
                margaret::framework::oidc_provider::authorization_handler::AuthorizationHandler<#view, #routes>;
        },
    )
}

#[must_use]
pub fn render_oidc_provider(
    items: &[OidcProviderItem],
    exchanger_issuers: &[&Tag],
    authorization: &DerivedAuthorization,
) -> Vec<GeneratedModuleTokens> {
    let exports = items.iter().map(|item| {
        let framework_path = item.framework_path();

        quote! { pub use #framework_path; }
    });
    let provider_endpoints = items
        .iter()
        .any(|item| item.uses_provider_endpoints())
        .then(|| {
            let module = format_ident!("{PROVIDER_ENDPOINTS_MODULE_NAME}");

            quote! { pub mod #module; }
        });
    let exchangers = (!exchanger_issuers.is_empty()).then(|| {
        let module = format_ident!("{SUBJECT_TOKEN_EXCHANGERS_MODULE_NAME}");

        quote! { pub mod #module; }
    });
    let authorization_handler = match authorization {
        DerivedAuthorization::Served(_) => {
            let module = format_ident!("{AUTHORIZATION_HANDLER_MODULE_NAME}");

            quote! { pub mod #module; }
        }
        DerivedAuthorization::Unserved => TokenStream::new(),
    };
    let mut modules = vec![GeneratedModuleTokens::new(
        OIDC_PROVIDER_MODULE_NAME,
        quote! { #authorization_handler #provider_endpoints #exchangers #(#exports)* },
    )];

    if let DerivedAuthorization::Served(served) = authorization {
        modules.push(authorization_handler_module(served));
    }

    if !exchanger_issuers.is_empty() {
        let submodules = exchanger_issuers.iter().map(|issuer| {
            let module = issuer.ident();

            quote! { pub mod #module; }
        });

        modules.push(GeneratedModuleTokens::new(
            format!("{OIDC_PROVIDER_MODULE_NAME}/{SUBJECT_TOKEN_EXCHANGERS_MODULE_NAME}"),
            quote! { #(#submodules)* },
        ));
        modules.extend(exchanger_issuers.iter().map(|issuer| {
            GeneratedModuleTokens::new(
                format!(
                    "{OIDC_PROVIDER_MODULE_NAME}/{SUBJECT_TOKEN_EXCHANGERS_MODULE_NAME}/{}",
                    issuer.ident().unraw()
                ),
                quote! {
                    pub use margaret::framework::subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
                },
            )
        }));
    }

    modules
}

#[cfg(test)]
mod tests {
    use syn::Path;

    use margaret_attributes::tag::Tag;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

    use super::render_oidc_provider;
    use crate::consent_page::ConsentPage;
    use crate::derived_authorization::DerivedAuthorization;
    use crate::oidc_provider_item::OidcProviderItem;
    use crate::served_authorization::ServedAuthorization;

    fn sources(modules: Vec<GeneratedModuleTokens>) -> Vec<String> {
        modules
            .into_iter()
            .map(|module| {
                let module = module.format().expect("the module formats");

                format!("{}:\n{}", module.name(), module.source())
            })
            .collect()
    }

    #[test]
    fn re_exports_every_provider_item_with_the_endpoints_and_exchangers() {
        assert_eq!(
            sources(render_oidc_provider(
                &OidcProviderItem::ALL,
                &[&Tag::from_path(&syn::parse_str::<Path>("ci").expect("the tag parses"))
                    .expect("the tag is plain")],
                &DerivedAuthorization::Served(ServedAuthorization {
                    consent: ConsentPage {
                        decision: RouteUrlInput {
                            path: "/authorize/consent".to_string(),
                            server: "public".to_string(),
                        },
                        view: CanonicalPath::new(vec![
                            "crate".to_string(),
                            "views".to_string(),
                            "ConsentView".to_string(),
                        ]),
                    },
                    url: "https://issuer.example/authorize".to_string(),
                }),
            )),
            vec![
                "oidc_provider:\npub mod authorization_handler;\npub mod provider_endpoints;\npub mod subject_token_exchangers;\npub use margaret::framework::accepted_clients::accepted_clients::AcceptedClients;\npub use margaret::framework::oidc_provider::authorization_endpoint::AuthorizationEndpoint;\npub use margaret::framework::oidc_provider::consent_endpoint::ConsentEndpoint;\npub use margaret::framework::oidc_provider::consent_handler::ConsentHandler;\npub use margaret::framework::oidc_provider::introspection_endpoint::IntrospectionEndpoint;\npub use margaret::framework::issuer_metadata::issuer_metadata::IssuerMetadata;\npub use margaret::framework::oidc_provider::provider_metadata_handler::ProviderMetadataHandler;\npub use margaret::framework::oidc_provider::revocation_endpoint::RevocationEndpoint;\npub use margaret::framework::subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;\npub use margaret::framework::oidc_provider::token_endpoint::TokenEndpoint;\npub use margaret::framework::oidc_provider::userinfo_endpoint::UserinfoEndpoint;\n".to_string(),
                "oidc_provider/authorization_handler:\npub type AuthorizationHandler = margaret::framework::oidc_provider::authorization_handler::AuthorizationHandler<\n    crate::views::ConsentView,\n    crate::margaret::routes::Routes,\n>;\n".to_string(),
                "oidc_provider/subject_token_exchangers:\npub mod ci;\n".to_string(),
                "oidc_provider/subject_token_exchangers/ci:\npub use margaret::framework::subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;\n".to_string(),
            ]
        );
    }

    #[test]
    fn re_exports_only_the_referenced_items() {
        assert_eq!(
            sources(render_oidc_provider(
                &[OidcProviderItem::TokenEndpoint],
                &[],
                &DerivedAuthorization::Unserved
            )),
            vec![
                "oidc_provider:\npub use margaret::framework::oidc_provider::token_endpoint::TokenEndpoint;\n".to_string(),
            ]
        );
    }
}
