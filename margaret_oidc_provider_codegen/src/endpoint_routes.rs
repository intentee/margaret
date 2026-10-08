use std::collections::BTreeSet;

use url::Url;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::derived_endpoint::DerivedEndpoint;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::oidc_provider_item::OidcProviderItem;
use crate::oidc_provider_item_path::oidc_provider_item_path;
use crate::provider_endpoint::ProviderEndpoint;

fn located_at_issuer(issuer: &IssuerIdentifier, path: &str) -> Url {
    let mut located = issuer.url().clone();

    located.set_path(path);

    located
}

pub(crate) struct EndpointRoutes<'plan> {
    pub(crate) bindings: &'plan ContainerBindings,
    pub(crate) locations: &'plan [RouteLocation<'plan>],
}

impl<'plan> EndpointRoutes<'plan> {
    pub(crate) fn path(
        &self,
        endpoint: ProviderEndpoint,
        server: &str,
    ) -> Result<String, OidcProviderCodegenError> {
        let mut paths = BTreeSet::new();

        for location in self
            .serving(endpoint)
            .into_iter()
            .filter(|location| location.server == server)
        {
            let path = location.path.pattern();

            if !endpoint.admits(location.method) {
                return Err(OidcProviderCodegenError::EndpointRouteMethod {
                    endpoint,
                    method: location.method,
                    path: path.to_string(),
                });
            }

            if location.path.parameters().next().is_some() {
                return Err(OidcProviderCodegenError::ParameterizedEndpointRoute {
                    endpoint,
                    path: path.to_string(),
                });
            }

            paths.insert(path.to_string());
        }

        match paths.first() {
            Some(path) if paths.len() == 1 => Ok(path.clone()),
            Some(_) => Err(OidcProviderCodegenError::AmbiguousEndpointRoute {
                endpoint,
                paths: paths.into_iter().collect(),
                server: server.to_string(),
            }),
            None => Err(OidcProviderCodegenError::MissingEndpointRoute {
                endpoint,
                server: server.to_string(),
            }),
        }
    }

    pub(crate) fn consent(
        &self,
        capable: &[ProviderEndpoint],
    ) -> Result<(), OidcProviderCodegenError> {
        let served = self.serves(&oidc_provider_item_path(OidcProviderItem::ConsentEndpoint));
        let authorization = capable.contains(&ProviderEndpoint::Authorization);

        if authorization == served {
            Ok(())
        } else if authorization {
            Err(OidcProviderCodegenError::MissingConsentRoute)
        } else {
            Err(OidcProviderCodegenError::UnconsumedConsentRoute)
        }
    }

    pub(crate) fn discovery_served(
        &self,
        server: &str,
        issuer: &IssuerIdentifier,
    ) -> Result<(), OidcProviderCodegenError> {
        let path = self.path(ProviderEndpoint::Discovery, server)?;
        let expected = oidc_discovery_url(issuer);

        if located_at_issuer(issuer, &path) == expected {
            Ok(())
        } else {
            Err(OidcProviderCodegenError::DiscoveryPathMismatch {
                expected: expected.path().to_string(),
                path,
            })
        }
    }

    pub(crate) fn url(
        &self,
        endpoint: ProviderEndpoint,
        server: &str,
        issuer: &IssuerIdentifier,
    ) -> Result<String, OidcProviderCodegenError> {
        self.path(endpoint, server)
            .map(|path| located_at_issuer(issuer, &path).to_string())
    }

    pub(crate) fn optional(
        &self,
        endpoint: ProviderEndpoint,
        capable: &[ProviderEndpoint],
        server: &str,
        issuer: &IssuerIdentifier,
    ) -> Result<DerivedEndpoint, OidcProviderCodegenError> {
        if capable.contains(&endpoint) {
            self.url(endpoint, server, issuer)
                .map(DerivedEndpoint::Served)
        } else if self.serving(endpoint).is_empty() {
            Ok(DerivedEndpoint::Unserved)
        } else {
            Err(OidcProviderCodegenError::UnconsumedEndpointRoute {
                capability: endpoint.capability(),
                endpoint,
            })
        }
    }

    pub(crate) fn provider_server(&self) -> Result<&'plan str, OidcProviderCodegenError> {
        let servers = self
            .serving(ProviderEndpoint::Discovery)
            .into_iter()
            .map(|location| location.server)
            .collect::<BTreeSet<&str>>();
        match servers.first() {
            Some(server) if servers.len() == 1 => Ok(server),
            Some(_) => Err(OidcProviderCodegenError::DiscoveryServedBySeveralServers {
                servers: servers.iter().map(ToString::to_string).collect(),
            }),
            None => Err(OidcProviderCodegenError::MissingDiscoveryRoute),
        }
    }

    pub(crate) fn serves(&self, handler: &CanonicalPath) -> bool {
        self.locations.iter().any(|location| {
            self.bindings
                .depends_directly_on(location.responder_path, handler)
        })
    }

    pub(crate) fn serving(&self, endpoint: ProviderEndpoint) -> Vec<&'plan RouteLocation<'plan>> {
        let handler = endpoint.handler_path();

        self.locations
            .iter()
            .filter(|location| {
                self.bindings
                    .depends_directly_on(location.responder_path, &handler)
            })
            .collect()
    }
}
