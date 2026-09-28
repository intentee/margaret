use std::sync::Arc;

use reqwest::header::AUTHORIZATION;
use reqwest::header::WWW_AUTHENTICATE;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;

use crate::admitting_handler::AdmittingHandler;

pub struct AdmissionAnswer {
    pub body: String,
    pub challenge: Option<String>,
    pub status: u16,
}

impl AdmissionAnswer {
    /// # Panics
    ///
    /// Panics when the fixture server cannot be reached.
    pub async fn request(verifier: BearerTokenVerifier, authorization: &str) -> Self {
        let fixture = TlsFixture::generate();
        let server = RunningFixtureServer::start(
            fixture.server_config.clone(),
            vec![RouteEntry::new(
                "/",
                vec![MethodHandler::anonymous(
                    "GET",
                    Arc::new(AdmittingHandler { verifier }),
                )],
            )],
        )
        .await;
        let response = fixture_client_builder(&fixture.certificate_authority)
            .build()
            .expect("the fixture client builds")
            .get(fixture.url(server.port(), "/"))
            .header(AUTHORIZATION, authorization)
            .send()
            .await
            .expect("the fixture server answers");
        let status = response.status().as_u16();
        let challenge = response.headers().get(WWW_AUTHENTICATE).map(|value| {
            value
                .to_str()
                .expect("the challenge is visible ascii")
                .to_string()
        });
        let body = response.text().await.expect("the body is text");

        server.stop().await;

        Self {
            body,
            challenge,
            status,
        }
    }
}
