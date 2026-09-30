use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoint_paths_module_name::PROVIDER_ENDPOINT_PATHS_MODULE_NAME;
use crate::provider_endpoint_routes::ProviderEndpointRoutes;

#[must_use]
pub fn render_provider_endpoint_paths(
    ProviderEndpointRoutes {
        authorization,
        discovery,
        introspection,
        jwks,
        revocation,
        token,
        userinfo,
        ..
    }: &ProviderEndpointRoutes,
) -> GeneratedModuleTokens {
    GeneratedModuleTokens::new(
        format!("{OIDC_PROVIDER_MODULE_NAME}/{PROVIDER_ENDPOINT_PATHS_MODULE_NAME}"),
        quote! {
            pub const PROVIDER_ENDPOINT_PATHS: margaret::framework::oidc_provider::provider_endpoint_paths::ProviderEndpointPaths =
                margaret::framework::oidc_provider::provider_endpoint_paths::ProviderEndpointPaths {
                    authorization: #authorization,
                    discovery: #discovery,
                    introspection: #introspection,
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
    use super::render_provider_endpoint_paths;
    use crate::provider_endpoint_routes::ProviderEndpointRoutes;

    #[test]
    fn renders_the_endpoint_paths_as_a_constant() {
        let module = render_provider_endpoint_paths(&ProviderEndpointRoutes {
            authorization: "/authorize".to_string(),
            discovery: "/.well-known/openid-configuration".to_string(),
            introspection: "/introspect".to_string(),
            jwks: "/jwks.json".to_string(),
            revocation: "/revoke".to_string(),
            server: "public".to_string(),
            token: "/token".to_string(),
            userinfo: "/userinfo".to_string(),
        })
        .format()
        .expect("the module formats");

        assert_eq!(module.name(), "oidc_provider/provider_endpoint_paths");
        assert_eq!(
            module.source(),
            "pub const PROVIDER_ENDPOINT_PATHS: margaret::framework::oidc_provider::provider_endpoint_paths::ProviderEndpointPaths = margaret::framework::oidc_provider::provider_endpoint_paths::ProviderEndpointPaths {\n    authorization: \"/authorize\",\n    discovery: \"/.well-known/openid-configuration\",\n    introspection: \"/introspect\",\n    jwks: \"/jwks.json\",\n    revocation: \"/revoke\",\n    token: \"/token\",\n    userinfo: \"/userinfo\",\n};\n"
        );
    }
}
