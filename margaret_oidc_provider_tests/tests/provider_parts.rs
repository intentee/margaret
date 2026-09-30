use std::sync::Arc;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_http::handler::Handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oidc_provider::introspection_endpoint::IntrospectionEndpoint;
use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;
use margaret_oidc_provider::provider_metadata_handler::ProviderMetadataHandler;
use margaret_oidc_provider::revocation_endpoint::RevocationEndpoint;
use margaret_oidc_provider::token_endpoint::TokenEndpoint;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_route_method::route_method::RouteMethod;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::introspection_route::IntrospectionRoute;
use crate::jwks_route::JwksRoute;
use crate::metadata_route::MetadataRoute;
use crate::revocation_route::RevocationRoute;
use crate::token_route::TokenRoute;
use crate::userinfo_route::UserinfoRoute;

fn route(path: &'static str, method: RouteMethod, handler: Arc<dyn Handler>) -> RouteEntry {
    RouteEntry::new(path, vec![MethodHandler::anonymous(method, handler)])
}

pub struct ProviderParts {
    pub clients: Arc<AcceptedClients>,
    pub endpoints: Arc<ProviderEndpoints>,
    pub issuance: Arc<dyn DeclaresTokenIssuance>,
    pub roller: Arc<JwksRoller>,
    pub secret_store: Arc<JwksSecretStore>,
    pub state: Arc<MemoryProviderState>,
}

impl ProviderParts {
    pub fn routes(&self, exchangers: Vec<Arc<SubjectTokenExchanger>>) -> Vec<RouteEntry> {
        vec![
            route(
                FIXTURE_ENDPOINT_PATHS.discovery,
                RouteMethod::Get,
                Arc::new(MetadataRoute {
                    handler: Arc::new(ProviderMetadataHandler::create(
                        &self.clients,
                        &self.endpoints,
                        self.issuance.as_ref(),
                    )),
                }),
            ),
            route(
                FIXTURE_ENDPOINT_PATHS.introspection,
                RouteMethod::Post,
                Arc::new(IntrospectionRoute {
                    endpoint: Arc::new(IntrospectionEndpoint::create(
                        Arc::clone(&self.clients),
                        Arc::clone(&self.secret_store),
                    )),
                }),
            ),
            route(
                FIXTURE_ENDPOINT_PATHS.jwks,
                RouteMethod::Get,
                Arc::new(JwksRoute {
                    handler: self.roller.public_jwks_handler(),
                }),
            ),
            route(
                FIXTURE_ENDPOINT_PATHS.revocation,
                RouteMethod::Post,
                Arc::new(RevocationRoute {
                    endpoint: Arc::new(RevocationEndpoint::create(
                        Arc::clone(&self.clients),
                        self.state.clone(),
                    )),
                }),
            ),
            route(
                FIXTURE_ENDPOINT_PATHS.token,
                RouteMethod::Post,
                Arc::new(TokenRoute {
                    endpoint: Arc::new(TokenEndpoint::create(
                        Arc::clone(&self.clients),
                        self.state.clone(),
                        Arc::new(SubjectTokenExchangers::create(exchangers)),
                        Arc::clone(&self.secret_store),
                        Arc::clone(&self.issuance),
                    )),
                }),
            ),
            route(
                FIXTURE_ENDPOINT_PATHS.userinfo,
                RouteMethod::Get,
                Arc::new(UserinfoRoute {
                    endpoint: Arc::new(UserinfoEndpoint::create(
                        Arc::clone(&self.secret_store),
                        self.issuance.as_ref(),
                    )),
                }),
            ),
        ]
    }
}
