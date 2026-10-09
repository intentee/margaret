use crate::served_endpoint::ServedEndpoint;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderEndpoints {
    pub authorization: ServedEndpoint,
    pub introspection: ServedEndpoint,
    pub issuer_origin: &'static str,
    pub jwks: &'static str,
    pub revocation: ServedEndpoint,
    pub token: &'static str,
    pub userinfo: ServedEndpoint,
}
