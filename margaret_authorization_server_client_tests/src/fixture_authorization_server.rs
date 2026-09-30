use std::sync::Arc;

use url::Url;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_http::handler::Handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oauth_client::declares_oauth_client::DeclaresOAuthClient;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_route_method::route_method::RouteMethod;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::localhost_discovery_metadata::localhost_discovery_metadata;
use crate::localhost_trust::localhost_trust;

pub struct FixtureAuthorizationServer {
    fixture: TlsFixture,
    server: RunningFixtureServer,
}

impl FixtureAuthorizationServer {
    pub async fn start(
        method: RouteMethod,
        endpoint_path: &'static str,
        endpoint: Arc<dyn Handler>,
    ) -> Self {
        let fixture = TlsFixture::generate();
        let server = RunningFixtureServer::start(
            fixture.server_config.clone(),
            vec![RouteEntry::new(
                endpoint_path,
                vec![MethodHandler::anonymous(method, endpoint)],
            )],
        )
        .await;

        Self { fixture, server }
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn client(&self, declaration: Arc<dyn DeclaresOAuthClient>) -> AuthorizationServerClient {
        let metadata = Arc::new(IssuerMetadata::awaiting());

        metadata.hold(localhost_discovery_metadata(
            AdvertisedEndpoint::Advertised(
                Url::parse("https://localhost/introspect")
                    .expect("the localhost introspection endpoint is a url"),
            ),
        ));

        AuthorizationServerClient::create(
            self.request_client(),
            Arc::clone(&metadata),
            Arc::new(TrustedIssuer::for_oidc_issuer(
                metadata,
                Arc::new(localhost_trust()),
            )),
            declaration,
        )
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn request_client(&self) -> Arc<IssuerRequestClient> {
        Arc::new(
            IssuerRequestClient::build(
                fixture_client_builder(&self.fixture.certificate_authority)
                    .resolve(&self.fixture.server_name, self.server.address()),
            )
            .expect("the fixture request client builds"),
        )
    }

    pub async fn stop(self) {
        self.server.stop().await;
    }
}
