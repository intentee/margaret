use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagKind {
    JwksClient,
    Middleware,
    OidcIssuer,
}

impl Display for TagKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let label = match self {
            TagKind::JwksClient => "jwks endpoint provider",
            TagKind::Middleware => "middleware handler",
            TagKind::OidcIssuer => "oidc issuer",
        };

        formatter.write_str(label)
    }
}
