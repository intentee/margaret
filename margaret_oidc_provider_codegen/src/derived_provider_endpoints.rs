use crate::derived_authorization::DerivedAuthorization;
use crate::derived_endpoint::DerivedEndpoint;

pub struct DerivedProviderEndpoints {
    pub authorization: DerivedAuthorization,
    pub introspection: DerivedEndpoint,
    pub issuer_origin: String,
    pub jwks: String,
    pub revocation: DerivedEndpoint,
    pub server: String,
    pub token: String,
    pub userinfo: DerivedEndpoint,
}
