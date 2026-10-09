use std::collections::BTreeSet;

use url::Url;

use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_route_parameter_codegen::literal_route_path::LiteralRoutePath;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;

use crate::consent_page::ConsentPage;
use crate::declared_consent::DeclaredConsent;
use crate::declared_consent_route::DeclaredConsentRoute;
use crate::declared_endpoint_routes::DeclaredEndpointRoutes;
use crate::derived_authorization::DerivedAuthorization;
use crate::derived_endpoint::DerivedEndpoint;
use crate::located_route::LocatedRoute;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::provider_endpoint::ProviderEndpoint;
use crate::served_authorization::ServedAuthorization;

fn located_at_issuer(issuer: &IssuerIdentifier, path: &str) -> Url {
    let mut located = issuer.url().clone();

    located.set_path(path);

    located
}

pub(crate) struct EndpointRoutes<'plan> {
    pub(crate) marked: &'plan DeclaredEndpointRoutes,
}

impl<'plan> EndpointRoutes<'plan> {
    pub(crate) fn authorization(
        &self,
        capable: &[ProviderEndpoint],
        server: &str,
        issuer: &IssuerIdentifier,
        sessions: &DeclaredSessions,
    ) -> Result<DerivedAuthorization, OidcProviderCodegenError> {
        match self.optional(ProviderEndpoint::Authorization, capable, server, issuer)? {
            DerivedEndpoint::Served(url) => {
                let consent = self.consent_page(server)?;

                match sessions {
                    DeclaredSessions::Issued(_) => {
                        Ok(DerivedAuthorization::Served(ServedAuthorization {
                            consent,
                            url,
                        }))
                    }
                    DeclaredSessions::Absent | DeclaredSessions::Consumed(_) => {
                        Err(OidcProviderCodegenError::AuthorizationWithoutSessions)
                    }
                }
            }
            DerivedEndpoint::Unserved => match self.marked.consent {
                DeclaredConsent::Declared(_) => {
                    Err(OidcProviderCodegenError::UnconsumedEndpointRoute {
                        capability: ProviderEndpoint::Consent.capability(),
                        endpoint: ProviderEndpoint::Consent,
                    })
                }
                DeclaredConsent::Undeclared => Ok(DerivedAuthorization::Unserved),
            },
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

    pub(crate) fn serving(&self, endpoint: ProviderEndpoint) -> Vec<LocatedRoute<'plan>> {
        self.marked
            .routes
            .iter()
            .filter(|marked| marked.endpoint == endpoint)
            .map(|marked| LocatedRoute {
                path: &marked.path,
                server: &marked.server,
            })
            .collect()
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

    fn consent_page(&self, server: &str) -> Result<ConsentPage, OidcProviderCodegenError> {
        let DeclaredConsent::Declared(DeclaredConsentRoute {
            path,
            route,
            server: consent_server,
            view,
            ..
        }) = &self.marked.consent
        else {
            return Err(OidcProviderCodegenError::MissingEndpointRoute {
                endpoint: ProviderEndpoint::Consent,
                server: server.to_string(),
            });
        };

        if consent_server != server {
            return Err(OidcProviderCodegenError::ConsentRouteOnForeignServer {
                provider_server: server.to_string(),
                route: route.to_string(),
                server: consent_server.clone(),
            });
        }

        match path.literal() {
            LiteralRoutePath::Literal(path) => Ok(ConsentPage {
                decision: RouteUrlInput {
                    path,
                    server: consent_server.clone(),
                },
                view: view.clone(),
            }),
            LiteralRoutePath::Parameterized => {
                Err(OidcProviderCodegenError::ParameterizedEndpointRoute {
                    endpoint: ProviderEndpoint::Consent,
                    path: path.pattern().to_string(),
                })
            }
        }
    }

    fn path(
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
            match location.path.literal() {
                LiteralRoutePath::Literal(literal) => paths.insert(literal),
                LiteralRoutePath::Parameterized => {
                    return Err(OidcProviderCodegenError::ParameterizedEndpointRoute {
                        endpoint,
                        path: location.path.pattern().to_string(),
                    });
                }
            };
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
}
