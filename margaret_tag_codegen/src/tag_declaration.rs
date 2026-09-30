use margaret_attributes::tag::Tag;

use crate::tag_kind::TagKind;
use crate::trusted_issuer_kind::TrustedIssuerKind;

pub(crate) enum TagDeclaration {
    Middleware,
    OAuthClient { issuer: Tag },
    TrustedIssuer(TrustedIssuerKind),
}

impl TagDeclaration {
    pub(crate) fn kind(&self) -> TagKind {
        match self {
            Self::Middleware => TagKind::Middleware,
            Self::OAuthClient { .. } => TagKind::OAuthClient,
            Self::TrustedIssuer(kind) => TagKind::TrustedIssuer(*kind),
        }
    }
}
