use crate::provider_route_paths::ProviderRoutePaths;

pub const FIXTURE_ENDPOINT_PATHS: ProviderRoutePaths = ProviderRoutePaths {
    authorization: "/authorize",
    discovery: "/.well-known/openid-configuration",
    introspection: "/introspect",
    jwks: "/jwks.json",
    revocation: "/revoke",
    token: "/token",
    userinfo: "/userinfo",
};
