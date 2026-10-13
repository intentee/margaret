#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustedIssuerKind {
    JwksEndpoint,
    OidcIssuer,
}
