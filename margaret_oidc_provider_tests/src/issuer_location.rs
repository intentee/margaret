use margaret_issuer_directory::discovered_issuer::DiscoveredIssuer;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

pub struct IssuerLocation {
    pub discovery_url: String,
    pub issuer: IssuerIdentifier,
}

impl IssuerLocation {
    #[must_use]
    pub fn of(issuer: IssuerIdentifier) -> Self {
        Self {
            discovery_url: oidc_discovery_url(&issuer).to_string(),
            issuer,
        }
    }

    #[must_use]
    pub fn discovered(&'static self) -> DiscoveredIssuer {
        DiscoveredIssuer {
            discovery_url: &self.discovery_url,
            issuer: self.issuer.as_str(),
        }
    }
}
