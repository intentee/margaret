use std::ops::ControlFlow;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use oauth2::ClientId;
use oauth2::EmptyExtraTokenFields;
use oauth2::StandardTokenIntrospectionResponse;
use oauth2::basic::BasicErrorResponseType;
use oauth2::basic::BasicTokenType;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_accepted_clients::registered_authentication::RegisteredAuthentication;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::no_store::no_store;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_validation::responded_to_form::responded_to_form;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_validation::validation_result::ValidationResult;

use crate::authenticated_client::authenticated_client;
use crate::oauth_error::oauth_error;
use crate::provider_error::ProviderError;
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
    introspection.set_iat(
        registered
            .iat
            .and_then(|iat| DateTime::from_timestamp(iat.seconds_since_epoch(), 0)),
    );
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

    /// # Errors
    ///
    /// Returns `ProviderError::ClientAuthentication` when the client assertion cannot be
    /// remembered.
    async fn respond(
        &self,
        request: &Request,
        submission: ValidationResult<TokenSubmission>,
    ) -> Result<Response, ProviderError> {
        let ValidationResult::Valid(TokenSubmission {
            client_authentication,
            token,
        }) = submission
        else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the introspection request is malformed",
            ));
        };
        let now = Utc::now();
        let registered = match authenticated_client(
            &self.clients,
            request,
            &client_authentication,
            NumericDate::from(now),
        )
        .await?
        {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(registered) => registered,
        };
        let RegisteredClient {
            authentication:
                RegisteredAuthentication::PrivateKeyJwt {
                    privileges:
                        ConfidentialPrivileges {
                            introspection: IntrospectionPermission::Permitted,
                            ..
                        },
                    ..
                },
            client,
            ..
        } = registered
        else {
            return Ok(oauth_error(
                403,
                BasicErrorResponseType::UnauthorizedClient,
                "the client may not introspect tokens",
            ));
        };

        Ok(no_store(Response::json(
            200,
            &match self
                .secret_store
                .verify_resource_access_token(&token, client.resources, now)
            {
                JwtVerification::Rejected(_) => {
                    StandardTokenIntrospectionResponse::<EmptyExtraTokenFields, BasicTokenType>::new(
                        false,
                        EmptyExtraTokenFields {},
                    )
                }
                JwtVerification::Verified(verified) => active_introspection(verified),
            },
        )))
    }
}

#[async_trait]
impl HandlesLimitedContent for IntrospectionEndpoint {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
        limit: BodyLimit,
    ) -> Result<ResponseContinuation, HandlerError> {
        responded_to_form(request, body, limit, |submission| {
            self.respond(request, submission)
        })
        .await
    }
}
