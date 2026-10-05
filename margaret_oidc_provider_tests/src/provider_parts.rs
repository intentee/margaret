use std::sync::Arc;

use serde_json::json;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_http::content_handler::ContentHandler;
use margaret_http::content_responder::content_responder;
use margaret_http::handler_future::HandlerFuture;
use margaret_http::head_handler::HeadHandler;
use margaret_http::head_responder::head_responder;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::responded::responded;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oidc_provider::introspection_endpoint::IntrospectionEndpoint;
use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;
use margaret_oidc_provider::provider_metadata_handler::ProviderMetadataHandler;
use margaret_oidc_provider::revocation_endpoint::RevocationEndpoint;
use margaret_oidc_provider::token_endpoint::TokenEndpoint;
use margaret_oidc_provider::userinfo_authentication::UserinfoAuthentication;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::form_answer::form_answer;

fn content_route(path: &'static str, handler: Arc<dyn ContentHandler>) -> RouteEntry {
    RouteEntry::new(
        path,
        vec![MethodHandler::content(ContentMethod::Post, handler)],
    )
}

fn head_route(path: &'static str, handler: Arc<dyn HeadHandler>) -> RouteEntry {
    RouteEntry::new(path, vec![MethodHandler::head(RouteMethod::Get, handler)])
}

fn introspection(endpoint: Arc<IntrospectionEndpoint>) -> Arc<dyn ContentHandler> {
    content_responder(
        endpoint,
        |endpoint: Arc<IntrospectionEndpoint>,
         request: &Request,
         body: RequestBody|
         -> HandlerFuture<'_> {
            Box::pin(form_answer(request, body, move |submission| async move {
                Ok(ResponseContinuation::from(
                    endpoint.respond(request, submission),
                ))
            }))
        },
    )
}

fn jwks(handler: Arc<PublicJwksHandler>) -> Arc<dyn HeadHandler> {
    head_responder(
        handler,
        |handler: Arc<PublicJwksHandler>, _request: &Request| -> HandlerFuture<'_> {
            Box::pin(async move { Ok(ResponseContinuation::from(handler.respond())) })
        },
    )
}

fn metadata(handler: Arc<ProviderMetadataHandler>) -> Arc<dyn HeadHandler> {
    head_responder(
        handler,
        |handler: Arc<ProviderMetadataHandler>, _request: &Request| -> HandlerFuture<'_> {
            Box::pin(async move { Ok(ResponseContinuation::from(handler.respond())) })
        },
    )
}

fn revocation(endpoint: Arc<RevocationEndpoint>) -> Arc<dyn ContentHandler> {
    content_responder(
        endpoint,
        |endpoint: Arc<RevocationEndpoint>,
         request: &Request,
         body: RequestBody|
         -> HandlerFuture<'_> {
            Box::pin(form_answer(request, body, move |submission| async move {
                responded(
                    endpoint
                        .respond(request, submission)
                        .await
                        .map_err(anyhow::Error::from),
                )
            }))
        },
    )
}

fn token(endpoint: Arc<TokenEndpoint>) -> Arc<dyn ContentHandler> {
    content_responder(
        endpoint,
        |endpoint: Arc<TokenEndpoint>, request: &Request, body: RequestBody| -> HandlerFuture<'_> {
            Box::pin(form_answer(
                request,
                body,
                move |token_request| async move {
                    responded(
                        endpoint
                            .respond(request, token_request)
                            .await
                            .map_err(anyhow::Error::from),
                    )
                },
            ))
        },
    )
}

fn userinfo(endpoint: Arc<UserinfoEndpoint>) -> Arc<dyn HeadHandler> {
    head_responder(
        endpoint,
        |endpoint: Arc<UserinfoEndpoint>, request: &Request| -> HandlerFuture<'_> {
            Box::pin(async move {
                responded(match endpoint.authenticate(request) {
                    UserinfoAuthentication::Authenticated(grant) => endpoint
                        .answer(&grant, &json!({"name": "Ada"}))
                        .map_err(anyhow::Error::from),
                    UserinfoAuthentication::Refused(refusal) => Ok(refusal),
                })
            })
        },
    )
}

pub struct ProviderParts {
    pub clients: Arc<AcceptedClients>,
    pub endpoints: Arc<ProviderEndpoints>,
    pub issuance: Arc<dyn DeclaresTokenIssuance>,
    pub roller: Arc<JwksRoller>,
    pub secret_store: Arc<JwksSecretStore>,
    pub state: Arc<dyn StoresProviderState>,
}

impl ProviderParts {
    /// # Panics
    ///
    /// Panics when the provider metadata cannot be serialized.
    #[must_use]
    pub fn routes(&self, exchangers: Vec<Arc<SubjectTokenExchanger>>) -> Vec<RouteEntry> {
        vec![
            head_route(
                FIXTURE_ENDPOINT_PATHS.discovery,
                metadata(Arc::new(
                    ProviderMetadataHandler::create(
                        &self.clients,
                        &self.endpoints,
                        self.issuance.as_ref(),
                    )
                    .expect("the provider metadata serializes"),
                )),
            ),
            content_route(
                FIXTURE_ENDPOINT_PATHS.introspection,
                introspection(Arc::new(IntrospectionEndpoint::create(
                    Arc::clone(&self.clients),
                    Arc::clone(&self.secret_store),
                ))),
            ),
            head_route(
                FIXTURE_ENDPOINT_PATHS.jwks,
                jwks(self.roller.public_jwks_handler()),
            ),
            content_route(
                FIXTURE_ENDPOINT_PATHS.revocation,
                revocation(Arc::new(RevocationEndpoint::create(
                    Arc::clone(&self.clients),
                    Arc::clone(&self.state),
                ))),
            ),
            content_route(
                FIXTURE_ENDPOINT_PATHS.token,
                token(Arc::new(TokenEndpoint::create(
                    Arc::clone(&self.clients),
                    Arc::clone(&self.state),
                    Arc::new(SubjectTokenExchangers::create(exchangers)),
                    Arc::clone(&self.secret_store),
                    Arc::clone(&self.issuance),
                ))),
            ),
            head_route(
                FIXTURE_ENDPOINT_PATHS.userinfo,
                userinfo(Arc::new(UserinfoEndpoint::create(
                    Arc::clone(&self.secret_store),
                    self.issuance.as_ref(),
                ))),
            ),
        ]
    }
}
