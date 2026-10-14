use std::convert::Infallible;
use std::sync::Arc;

use serde_json::json;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_database::database::Database;
use margaret_http::body_limit::BodyLimit;
use margaret_http::content_handler::ContentHandler;
use margaret_http::head_handler::HeadHandler;
use margaret_http::limited_content_handler::limited_content_handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http::terminal_head_handler::terminal_head_handler;
use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_provider::introspection_endpoint::IntrospectionEndpoint;
use margaret_oidc_provider::provider_metadata_handler::ProviderMetadataHandler;
use margaret_oidc_provider::revocation_endpoint::RevocationEndpoint;
use margaret_oidc_provider::token_endpoint::TokenEndpoint;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::constants::FIXTURE_ACCEPTED_RESOURCES;
use crate::constants::FIXTURE_ENDPOINT_PATHS;
use crate::constants::FIXTURE_PROVIDER_SUPPORT;
use crate::fixed_userinfo_claims::FixedUserinfoClaims;

const FORM_LIMIT: BodyLimit = BodyLimit::new(16_384);

fn content_route(handler: Arc<dyn ContentHandler<Infallible>>) -> RouteEntry<Infallible> {
    RouteEntry::new(vec![MethodHandler::content(ContentMethod::Post, handler)])
}

fn head_route(handler: Arc<dyn HeadHandler<Infallible>>) -> RouteEntry<Infallible> {
    RouteEntry::new(vec![MethodHandler::head(RouteMethod::Get, handler)])
}

pub struct ProviderParts {
    pub clients: Arc<AcceptedClients>,
    pub database: Arc<Database>,
    pub endpoints: ProviderEndpoints,
    pub issuance: TokenIssuance,
    pub public_jwks_handler: Arc<PublicJwksHandler>,
    pub secret_store: Arc<JwksSecretStore>,
}

impl ProviderParts {
    pub const PATHS: [&'static str; 6] = [
        FIXTURE_ENDPOINT_PATHS.discovery,
        FIXTURE_ENDPOINT_PATHS.introspection,
        FIXTURE_ENDPOINT_PATHS.jwks,
        FIXTURE_ENDPOINT_PATHS.revocation,
        FIXTURE_ENDPOINT_PATHS.token,
        FIXTURE_ENDPOINT_PATHS.userinfo,
    ];

    /// # Panics
    ///
    /// Panics when the provider metadata cannot be serialized.
    #[must_use]
    pub fn routes(
        &self,
        exchangers: Vec<Arc<SubjectTokenExchanger>>,
    ) -> [RouteEntry<Infallible>; 6] {
        [
            head_route(terminal_head_handler(Arc::new(
                ProviderMetadataHandler::create(
                    FIXTURE_PROVIDER_SUPPORT,
                    self.endpoints,
                    self.issuance,
                )
                .expect("the provider metadata serializes"),
            ))),
            content_route(limited_content_handler(
                Arc::new(IntrospectionEndpoint::create(
                    Arc::clone(&self.clients),
                    Arc::clone(&self.secret_store),
                )),
                FORM_LIMIT,
            )),
            head_route(terminal_head_handler(Arc::clone(&self.public_jwks_handler))),
            content_route(limited_content_handler(
                Arc::new(RevocationEndpoint::create(
                    Arc::clone(&self.clients),
                    Arc::clone(&self.secret_store),
                    Arc::clone(&self.database),
                    FIXTURE_ACCEPTED_RESOURCES,
                )),
                FORM_LIMIT,
            )),
            content_route(limited_content_handler(
                Arc::new(TokenEndpoint::create(
                    Arc::clone(&self.clients),
                    Arc::new(SubjectTokenExchangers::create(exchangers)),
                    Arc::clone(&self.secret_store),
                    self.issuance,
                )),
                FORM_LIMIT,
            )),
            head_route(terminal_head_handler(Arc::new(UserinfoEndpoint::create(
                Arc::clone(&self.secret_store),
                self.issuance,
                Arc::new(FixedUserinfoClaims {
                    claims: json!({"name": "Ada"}),
                }),
            )))),
        ]
    }
}
