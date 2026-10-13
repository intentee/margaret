use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::trusted_issuer_kind::TrustedIssuerKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagKind {
    AdmittedClient,
    Middleware,
    OAuthClient,
    ResourceTokens,
    TokenIssuance,
    TrustedIssuer(TrustedIssuerKind),
}

impl Display for TagKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let label = match self {
            TagKind::AdmittedClient => "client admitted by the provider",
            TagKind::Middleware => "middleware handler",
            TagKind::OAuthClient => "oauth client",
            TagKind::ResourceTokens => "resource",
            TagKind::TokenIssuance => "token issuance",
            TagKind::TrustedIssuer(TrustedIssuerKind::JwksEndpoint) => {
                "trusted issuer with published keys"
            }
            TagKind::TrustedIssuer(TrustedIssuerKind::OidcIssuer) => {
                "trusted issuer with discovered keys"
            }
        };

        formatter.write_str(label)
    }
}
