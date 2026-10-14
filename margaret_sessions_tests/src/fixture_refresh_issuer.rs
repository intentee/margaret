use std::convert::Infallible;
use std::sync::Arc;

use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::redirecting_handler::RedirectingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_route_method::route_method::RouteMethod;

const TEMPORARY_REDIRECT: u16 = 307;

async fn serving(refresh: Arc<dyn HeadHandler<Infallible>>) -> FixtureRefreshIssuer {
    let tls = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        tls.server_config.clone(),
        ["/sessions/refresh"],
        [RouteEntry::new(vec![MethodHandler::head(
            RouteMethod::Post,
            refresh,
        )])],
    )
    .await;

    FixtureRefreshIssuer { server, tls }
}

pub struct FixtureRefreshIssuer {
    pub server: RunningFixtureServer,
    pub tls: TlsFixture,
}

impl FixtureRefreshIssuer {
    pub async fn answering(status: u16, body: Vec<u8>) -> Self {
        serving(Arc::new(StaticHandler {
            body,
            content_type: "application/json",
            status,
        }))
        .await
    }

    pub async fn redirecting(location: String) -> Self {
        serving(Arc::new(RedirectingHandler {
            location,
            status: TEMPORARY_REDIRECT,
        }))
        .await
    }
}
