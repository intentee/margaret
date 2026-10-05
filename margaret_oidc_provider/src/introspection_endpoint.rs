use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use oauth2::ClientId;
use oauth2::EmptyExtraTokenFields;
use oauth2::StandardTokenIntrospectionResponse;
use oauth2::basic::BasicErrorResponseType;
use oauth2::basic::BasicTokenType;

use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_validation::validation_result::ValidationResult;

use crate::authenticated_client::authenticated_client;
use crate::no_store::no_store;
use crate::oauth_error::oauth_error;
use crate::token_submission::TokenSubmission;

fn active_introspection<TProfile>(
    VerifiedJwt {
        claims:
            ResourceAccessTokenClaims {
                client_id,
                scope,
                subject,
            },
        registered,
        ..
    }: VerifiedJwt<ResourceAccessTokenClaims, TProfile>,
) -> StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType> {
    let mut introspection = StandardTokenIntrospectionResponse::new(true, EmptyExtraTokenFields {});

    introspection.set_aud(Some(match registered.aud {
        AudienceClaim::Multiple(audiences) => audiences,
        AudienceClaim::Single(audience) => vec![audience],
    }));
    introspection.set_client_id(Some(ClientId::new(client_id.as_str().to_string())));
    introspection.set_exp(DateTime::from_timestamp(
        registered.exp.seconds_since_epoch(),
        0,
    ));
    introspection.set_iat(DateTime::from_timestamp(
        registered.iat.seconds_since_epoch(),
        0,
    ));
    introspection.set_iss(Some(registered.iss));
    introspection.set_scopes(Some(scope.scopes.iter().map(oauth2::Scope::from).collect()));
    introspection.set_sub(Some(subject));
    introspection.set_token_type(Some(BasicTokenType::Bearer));

    introspection
}

pub struct IntrospectionEndpoint {
    clients: Arc<AcceptedClients>,
    secret_store: Arc<JwksSecretStore>,
}

impl IntrospectionEndpoint {
    #[must_use]
    pub fn create(clients: Arc<AcceptedClients>, secret_store: Arc<JwksSecretStore>) -> Self {
        Self {
            clients,
            secret_store,
        }
    }

    #[must_use]
    pub fn respond(
        &self,
        request: &Request,
        submission: ValidationResult<TokenSubmission>,
    ) -> Response {
        let ValidationResult::Valid(TokenSubmission {
            client_id, token, ..
        }) = submission
        else {
            return oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the introspection request is malformed",
            );
        };
        let client = match authenticated_client(&self.clients, request, client_id.as_deref()) {
            ControlFlow::Break(refusal) => return refusal,
            ControlFlow::Continue(client) => client,
        };
        let AcceptedClientAuthentication::ClientSecretBasic {
            privileges:
                ConfidentialPrivileges {
                    introspection: IntrospectionPermission::Permitted,
                    ..
                },
            ..
        } = &client.authentication
        else {
            return oauth_error(
                403,
                BasicErrorResponseType::UnauthorizedClient,
                "the client may not introspect tokens",
            );
        };

        no_store(Response::json(
            200,
            &match self.secret_store.verify_resource_access_token(
                &token,
                client.resources.members(),
                Utc::now(),
            ) {
                JwtVerification::Rejected(_) => {
                    StandardTokenIntrospectionResponse::<EmptyExtraTokenFields, BasicTokenType>::new(
                        false,
                        EmptyExtraTokenFields {},
                    )
                }
                JwtVerification::Verified(verified) => active_introspection(verified),
            },
        ))
    }
}
