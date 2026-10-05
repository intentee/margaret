use margaret_oidc_provider::provider_endpoint_paths::ProviderEndpointPaths;

pub const FIXTURE_ENDPOINT_PATHS: ProviderEndpointPaths = ProviderEndpointPaths {
    authorization: "/authorize",
    discovery: "/.well-known/openid-configuration",
    introspection: "/introspect",
    jwks: "/jwks.json",
    revocation: "/revoke",
    token: "/token",
    userinfo: "/userinfo",
};
