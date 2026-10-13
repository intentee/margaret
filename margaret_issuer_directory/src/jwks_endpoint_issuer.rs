#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JwksEndpointIssuer {
    pub issuer: &'static str,
    pub jwks_uri: &'static str,
}
