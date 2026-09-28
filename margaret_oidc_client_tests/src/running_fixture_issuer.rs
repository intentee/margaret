use reqwest::ClientBuilder;

use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;

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

    pub fn client_builder(&self) -> ClientBuilder {
        fixture_client_builder(&self.fixture.certificate_authority)
            .resolve(&self.fixture.server_name, self.server.address())
    }

    pub async fn stop(self) {
        self.server.stop().await;
    }
}
