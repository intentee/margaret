use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::trusted_issuer_kind::TrustedIssuerKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagKind {
    Middleware,
    OAuthClient,
    TrustedIssuer(TrustedIssuerKind),
}

impl Display for TagKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let label = match self {
            TagKind::Middleware => "middleware handler",
            TagKind::OAuthClient => "oauth client",
            TagKind::TrustedIssuer(TrustedIssuerKind::JwksEndpoint) => "jwks endpoint provider",
            TagKind::TrustedIssuer(TrustedIssuerKind::OidcIssuer) => "oidc issuer",
        };

        formatter.write_str(label)
    }
}
