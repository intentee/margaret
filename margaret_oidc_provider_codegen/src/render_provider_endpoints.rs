use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::derived_provider_endpoints::DerivedProviderEndpoints;
use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoints_module_name::PROVIDER_ENDPOINTS_MODULE_NAME;

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
) -> GeneratedModuleTokens {
    GeneratedModuleTokens::new(
        format!("{OIDC_PROVIDER_MODULE_NAME}/{PROVIDER_ENDPOINTS_MODULE_NAME}"),
        quote! {
            pub const PROVIDER_ENDPOINTS: margaret::framework::oidc_provider::provider_endpoints::ProviderEndpoints =
                margaret::framework::oidc_provider::provider_endpoints::ProviderEndpoints {
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
}

#[cfg(test)]
mod tests {
    use super::render_provider_endpoints;
    use crate::derived_provider_endpoints::DerivedProviderEndpoints;

    #[test]
    fn renders_the_endpoints_as_a_constant() {
        let module = render_provider_endpoints(&DerivedProviderEndpoints {
            authorization: "https://issuer.example/authorize".to_string(),
            introspection: "https://issuer.example/introspect".to_string(),
            issuer_origin: "https://issuer.example".to_string(),
            jwks: "https://issuer.example/jwks.json".to_string(),
            revocation: "https://issuer.example/revoke".to_string(),
            server: "public".to_string(),
            token: "https://issuer.example/token".to_string(),
            userinfo: "https://issuer.example/userinfo".to_string(),
        })
        .format()
        .expect("the module formats");

        assert_eq!(module.name(), "oidc_provider/provider_endpoints");
        assert_eq!(
            module.source(),
            "pub const PROVIDER_ENDPOINTS: margaret::framework::oidc_provider::provider_endpoints::ProviderEndpoints = margaret::framework::oidc_provider::provider_endpoints::ProviderEndpoints {\n    authorization: \"https://issuer.example/authorize\",\n    introspection: \"https://issuer.example/introspect\",\n    issuer_origin: \"https://issuer.example\",\n    jwks: \"https://issuer.example/jwks.json\",\n    revocation: \"https://issuer.example/revoke\",\n    token: \"https://issuer.example/token\",\n    userinfo: \"https://issuer.example/userinfo\",\n};\n"
        );
    }
}
