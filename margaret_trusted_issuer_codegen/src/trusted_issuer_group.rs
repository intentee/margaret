use std::iter;

use url::Url;

use margaret_https_url::https_url::HttpsUrl;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::declared_trust::DeclaredTrust;
use crate::trust_declaration::TrustDeclaration;
use crate::trust_source::TrustSource;
use crate::trusted_issuer_codegen_error::TrustedIssuerCodegenError;

pub enum TrustedIssuerGroup<'index> {
    Discovered {
        discovery_url: Url,
        issuer: IssuerIdentifier,
        lead: DeclaredTrust<'index>,
        others: Vec<DeclaredTrust<'index>>,
    },
    JwksEndpoint {
        issuer: IssuerIdentifier,
        jwks_uri: HttpsUrl,
        trust: DeclaredTrust<'index>,
    },
}

impl<'index> TrustedIssuerGroup<'index> {
    pub(crate) fn founded_by(
        TrustDeclaration {
            issuer,
            source,
            trust,
        }: TrustDeclaration<'index>,
    ) -> Self {
        match source {
            TrustSource::Discovery => Self::Discovered {
                discovery_url: oidc_discovery_url(&issuer),
                issuer,
                lead: trust,
                others: Vec::new(),
            },
            TrustSource::JwksEndpoint { jwks_uri } => Self::JwksEndpoint {
                issuer,
                jwks_uri,
                trust,
            },
        }
    }

    #[must_use]
    pub fn issuer(&self) -> &IssuerIdentifier {
        match self {
            Self::Discovered { issuer, .. } | Self::JwksEndpoint { issuer, .. } => issuer,
        }
    }

    #[must_use]
    pub fn lead(&self) -> &DeclaredTrust<'index> {
        match self {
            Self::Discovered { lead, .. } => lead,
            Self::JwksEndpoint { trust, .. } => trust,
        }
    }

    pub fn members(&self) -> impl Iterator<Item = &DeclaredTrust<'index>> {
        let others: &[DeclaredTrust<'index>] = match self {
            Self::Discovered { others, .. } => others,
            Self::JwksEndpoint { .. } => &[],
        };

        iter::once(self.lead()).chain(others)
    }

    pub(crate) fn admit(
        &mut self,
        TrustDeclaration { source, trust, .. }: TrustDeclaration<'index>,
    ) -> Result<(), TrustedIssuerCodegenError> {
        if let Some(member) = self
            .members()
            .find(|member| member.audience.as_str() == trust.audience.as_str())
        {
            return Err(TrustedIssuerCodegenError::TokenTrustDeclaredTwice {
                audience: trust.audience.as_str().to_string(),
                first: member.anchor.canonical_path().to_string(),
                issuer: self.issuer().as_str().to_string(),
                second: trust.anchor.canonical_path().to_string(),
            });
        }

        if let Self::Discovered { others, .. } = self
            && matches!(source, TrustSource::Discovery)
        {
            others.push(trust);

            return Ok(());
        }

        Err(TrustedIssuerCodegenError::JwksEndpointIssuerTrustedTwice {
            first: self.lead().anchor.canonical_path().to_string(),
            issuer: self.issuer().as_str().to_string(),
            second: trust.anchor.canonical_path().to_string(),
        })
    }
}
