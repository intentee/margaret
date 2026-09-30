use url::Origin;
use url::Url;

use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::provider_endpoint_paths::ProviderEndpointPaths;
use crate::provider_error::ProviderError;

fn located_at_issuer(issuer: &IssuerIdentifier, path: &str) -> Url {
    let mut located = issuer.url().clone();

    located.set_path(path);

    located
}

pub struct ProviderEndpoints {
    pub authorization: Url,
    pub introspection: Url,
    pub issuer_origin: Origin,
    pub jwks: Url,
    pub revocation: Url,
    pub token: Url,
    pub userinfo: Url,
}

impl ProviderEndpoints {
    /// # Errors
    ///
    /// Returns `ProviderError::DiscoveryPathMismatch` when the discovery route is not served at
    /// the discovery location of the issuer.
    pub fn create(
        issuance: &dyn DeclaresTokenIssuance,
        paths: ProviderEndpointPaths,
    ) -> Result<Self, ProviderError> {
        let issuer = &issuance.token_issuance().issuer;
        let discovery = oidc_discovery_url(issuer);

        if located_at_issuer(issuer, paths.discovery) != discovery {
            return Err(ProviderError::DiscoveryPathMismatch {
                discovery_path: paths.discovery,
                expected_location: Box::new(discovery),
            });
        }

        Ok(Self {
            authorization: located_at_issuer(issuer, paths.authorization),
            introspection: located_at_issuer(issuer, paths.introspection),
            issuer_origin: issuer.url().origin(),
            jwks: located_at_issuer(issuer, paths.jwks),
            revocation: located_at_issuer(issuer, paths.revocation),
            token: located_at_issuer(issuer, paths.token),
            userinfo: located_at_issuer(issuer, paths.userinfo),
        })
    }

    /// # Errors
    ///
    /// Returns `ProviderError::ServerUrlMalformed` when the url of the server that serves the
    /// provider is not a url, and `ProviderError::IssuerNotServed` when the issuer is not served
    /// at the origin of that server.
    pub fn served_by(&self, server_url: &str) -> Result<(), ProviderError> {
        let server_origin = Url::parse(server_url)
            .map_err(|source| ProviderError::ServerUrlMalformed {
                server_url: server_url.to_string(),
                source,
            })?
            .origin();

        if server_origin == self.issuer_origin {
            Ok(())
        } else {
            Err(ProviderError::IssuerNotServed {
                issuer_origin: self.issuer_origin.ascii_serialization(),
                server_origin: server_origin.ascii_serialization(),
            })
        }
    }
}
