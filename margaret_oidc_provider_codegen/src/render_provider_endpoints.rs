use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::authorization_endpoint_url_module_name::AUTHORIZATION_ENDPOINT_URL_MODULE_NAME;
use crate::derived_authorization::DerivedAuthorization;
use crate::derived_endpoint::DerivedEndpoint;
use crate::derived_provider_endpoints::DerivedProviderEndpoints;
use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoints_module_name::PROVIDER_ENDPOINTS_MODULE_NAME;
use crate::served_authorization::ServedAuthorization;

fn served(endpoint: &TokenStream) -> TokenStream {
    quote! { margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Served(#endpoint) }
}

fn derived(endpoint: &DerivedEndpoint) -> TokenStream {
    match endpoint {
        DerivedEndpoint::Served(url) => served(&quote! { #url }),
        DerivedEndpoint::Unserved => {
            quote! { margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Unserved }
        }
    }
}

#[must_use]
pub fn render_provider_endpoints(
    DerivedProviderEndpoints {
        authorization,
        introspection,
        issuer_origin,
        jwks,
        revocation,
        token,
        userinfo,
        ..
    }: &DerivedProviderEndpoints,
) -> Vec<GeneratedModuleTokens> {
    let module = format!("{OIDC_PROVIDER_MODULE_NAME}/{PROVIDER_ENDPOINTS_MODULE_NAME}");
    let introspection = derived(introspection);
    let revocation = derived(revocation);
    let userinfo = derived(userinfo);
    let endpoints = |authorization: TokenStream, declarations: TokenStream| {
        GeneratedModuleTokens::new(
            module.clone(),
            quote! {
                #declarations

                pub const PROVIDER_ENDPOINTS: margaret::framework::oidc_discovery::provider_endpoints::ProviderEndpoints =
                    margaret::framework::oidc_discovery::provider_endpoints::ProviderEndpoints {
                        authorization: #authorization,
                        introspection: #introspection,
                        issuer_origin: #issuer_origin,
                        jwks: #jwks,
                        revocation: #revocation,
                        token: #token,
                        userinfo: #userinfo,
                    };
            },
        )
    };

    match authorization {
        DerivedAuthorization::Served(ServedAuthorization { url, .. }) => {
            let url_module = format_ident!("{AUTHORIZATION_ENDPOINT_URL_MODULE_NAME}");

            vec![
                endpoints(
                    served(&quote! { #url_module::AUTHORIZATION_ENDPOINT_URL }),
                    quote! { pub mod #url_module; },
                ),
                GeneratedModuleTokens::new(
                    format!("{module}/{AUTHORIZATION_ENDPOINT_URL_MODULE_NAME}"),
                    quote! { pub const AUTHORIZATION_ENDPOINT_URL: &str = #url; },
                ),
            ]
        }
        DerivedAuthorization::Unserved => vec![endpoints(
            derived(&DerivedEndpoint::Unserved),
            TokenStream::new(),
        )],
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

    use super::render_provider_endpoints;
    use crate::consent_page::ConsentPage;
    use crate::derived_authorization::DerivedAuthorization;
    use crate::derived_endpoint::DerivedEndpoint;
    use crate::derived_provider_endpoints::DerivedProviderEndpoints;
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

    fn endpoints(served: bool) -> DerivedProviderEndpoints {
        let endpoint = |url: &str| {
            if served {
                DerivedEndpoint::Served(url.to_string())
            } else {
                DerivedEndpoint::Unserved
            }
        };

        DerivedProviderEndpoints {
            authorization: if served {
                DerivedAuthorization::Served(ServedAuthorization {
                    consent: ConsentPage {
                        decision: RouteUrlInput {
                            path: "/authorize/consent".to_string(),
                            server: "public".to_string(),
                        },
                        view: CanonicalPath::new(vec![
                            "crate".to_string(),
                            "ConsentView".to_string(),
                        ]),
                    },
                    url: "https://issuer.example/authorize".to_string(),
                })
            } else {
                DerivedAuthorization::Unserved
            },
            introspection: endpoint("https://issuer.example/introspect"),
            issuer_origin: "https://issuer.example".to_string(),
            jwks: "https://issuer.example/jwks.json".to_string(),
            revocation: endpoint("https://issuer.example/revoke"),
            server: "public".to_string(),
            token: "https://issuer.example/token".to_string(),
            userinfo: endpoint("https://issuer.example/userinfo"),
        }
    }

    #[test]
    fn renders_the_served_endpoints_and_the_authorization_endpoint_url() {
        assert_eq!(
            sources(render_provider_endpoints(&endpoints(true))),
            vec![
                "oidc_provider/provider_endpoints:\npub mod authorization_endpoint_url;\npub const PROVIDER_ENDPOINTS: margaret::framework::oidc_discovery::provider_endpoints::ProviderEndpoints = margaret::framework::oidc_discovery::provider_endpoints::ProviderEndpoints {\n    authorization: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Served(\n        authorization_endpoint_url::AUTHORIZATION_ENDPOINT_URL,\n    ),\n    introspection: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Served(\n        \"https://issuer.example/introspect\",\n    ),\n    issuer_origin: \"https://issuer.example\",\n    jwks: \"https://issuer.example/jwks.json\",\n    revocation: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Served(\n        \"https://issuer.example/revoke\",\n    ),\n    token: \"https://issuer.example/token\",\n    userinfo: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Served(\n        \"https://issuer.example/userinfo\",\n    ),\n};\n".to_string(),
                "oidc_provider/provider_endpoints/authorization_endpoint_url:\npub const AUTHORIZATION_ENDPOINT_URL: &str = \"https://issuer.example/authorize\";\n".to_string(),
            ]
        );
    }

    #[test]
    fn renders_the_endpoints_the_provider_does_not_serve_as_unserved() {
        assert_eq!(
            sources(render_provider_endpoints(&endpoints(false))),
            vec![
                "oidc_provider/provider_endpoints:\npub const PROVIDER_ENDPOINTS: margaret::framework::oidc_discovery::provider_endpoints::ProviderEndpoints = margaret::framework::oidc_discovery::provider_endpoints::ProviderEndpoints {\n    authorization: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Unserved,\n    introspection: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Unserved,\n    issuer_origin: \"https://issuer.example\",\n    jwks: \"https://issuer.example/jwks.json\",\n    revocation: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Unserved,\n    token: \"https://issuer.example/token\",\n    userinfo: margaret::framework::oidc_discovery::served_endpoint::ServedEndpoint::Unserved,\n};\n".to_string(),
            ]
        );
    }
}
