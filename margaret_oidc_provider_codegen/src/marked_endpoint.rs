#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MarkedEndpoint {
    Authorization,
    Consent,
    Discovery,
    Introspection,
    Jwks,
    Revocation,
    Token,
    Userinfo,
}
