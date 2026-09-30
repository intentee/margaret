#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderEndpointPaths {
    pub authorization: &'static str,
    pub discovery: &'static str,
    pub introspection: &'static str,
    pub jwks: &'static str,
    pub revocation: &'static str,
    pub token: &'static str,
    pub userinfo: &'static str,
}
