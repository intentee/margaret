use std::net::SocketAddr;

use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

use crate::fixture_issuer_routes::FixtureIssuerRoutes;

pub struct RunningFixtureIssuer {
    fixture: TlsFixture,
    server: RunningFixtureServer,
}

impl RunningFixtureIssuer {
    pub async fn start(routes: FixtureIssuerRoutes) -> Self {
        let fixture = TlsFixture::generate();
        let server =
            RunningFixtureServer::start(fixture.server_config.clone(), routes.into_route_entries())
                .await;

        Self { fixture, server }
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn request_client(&self) -> IssuerRequestClient {
        self.request_client_resolving_to(self.server.address())
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn request_client_resolving_to(&self, address: SocketAddr) -> IssuerRequestClient {
        IssuerRequestClient::build(
            fixture_client_builder(&self.fixture.certificate_authority)
                .resolve(&self.fixture.server_name, address),
        )
        .expect("the fixture request client builds")
    }

    pub async fn stop(self) {
        self.server.stop().await;
    }
}
