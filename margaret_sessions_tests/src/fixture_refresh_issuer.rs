use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_route_method::route_method::RouteMethod;

pub struct FixtureRefreshIssuer {
    pub server: RunningFixtureServer,
    pub tls: TlsFixture,
}

impl FixtureRefreshIssuer {
    pub async fn answering(status: u16, body: Vec<u8>) -> Self {
        let tls = TlsFixture::generate();
        let server = RunningFixtureServer::start(
            tls.server_config.clone(),
            vec![RouteEntry::new(
                "/sessions/refresh",
                vec![MethodHandler::head(
                    RouteMethod::Post,
                    Arc::new(StaticHandler {
                        body,
                        content_type: "application/json",
                        status,
                    }),
                )],
            )],
        )
        .await;

        Self { server, tls }
    }
}
