use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use futures_util::TryFutureExt;
use futures_util::future;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;
use oauth2::basic::BasicErrorResponseType;
use url::Url;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_identity_session::id_token_claims::IdTokenClaims;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::presented_code::PresentedCode;
use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret_validation::validation_result::ValidationResult;

use crate::authenticated_client::authenticated_client;
use crate::code_admission::CodeAdmission;
use crate::oauth_error::oauth_error;
use crate::prepared_tokens::PreparedTokens;
use crate::provider_error::ProviderError;
use crate::provider_token_fields::ProviderTokenFields;
use crate::refresh_token_issue::RefreshTokenIssue;
use crate::requested_scope::RequestedScope;
use crate::target_resource::target_resource;
use crate::token_issue::TokenIssue;
use crate::token_request::TokenRequest;

fn invalid_grant(description: &'static str) -> Response {
    oauth_error(400, BasicErrorResponseType::InvalidGrant, description)
}

fn refused_subject_token(refusal: &SubjectTokenRefusal) -> Response {
    oauth_error(
        400,
        BasicErrorResponseType::InvalidRequest,
        match refusal {
            SubjectTokenRefusal::Ambiguous { .. } => {
                "the subject token is addressed to more than one exchanger"
            }
            SubjectTokenRefusal::ExchangeRefused => "the exchanger refused the subject token",
            SubjectTokenRefusal::Misaddressed { .. } => {
                "the subject token is addressed to no exchanger"
            }
            SubjectTokenRefusal::Rejected(_) => "the subject token fails verification",
            SubjectTokenRefusal::TokenTypeMismatch => {
                "the subject token type does not match the profile of its exchanger"
            }
            SubjectTokenRefusal::UntrustedIssuer { .. } => {
                "the subject token issuer is trusted by no exchanger"
            }
        },
    )
}

fn replayed_code() -> Response {
    invalid_grant("the authorization code was already redeemed, so its tokens are revoked")
}

fn replayed_refresh_token() -> Response {
    invalid_grant("the refresh token was already rotated, so its family is revoked")
}

fn unknown_code() -> Response {
    invalid_grant("the authorization code is not known")
}

fn unknown_refresh_token() -> Response {
    invalid_grant("the refresh token is not known")
}

fn unauthorized_client() -> Response {
    oauth_error(
        400,
        BasicErrorResponseType::UnauthorizedClient,
        "the client may not use the grant type",
    )
}

fn code_settled(spending: CodeSpending, spent: impl FnOnce() -> Response) -> Response {
    match spending {
        CodeSpending::Expired => unknown_code(),
        CodeSpending::Replayed => replayed_code(),
        CodeSpending::Spent => spent(),
    }
}

fn refresh_rotated(rotation: RefreshRotation, prepared: PreparedTokens) -> Response {
    match rotation {
        RefreshRotation::Replayed => replayed_refresh_token(),
        RefreshRotation::Revoked => unknown_refresh_token(),
        RefreshRotation::Rotated => prepared.response(),
    }
}

fn redeemed_issue(
    policy: &CodeGrantPolicy,
    AuthorizationGrant {
        scopes, subject, ..
    }: AuthorizationGrant,
    resource: Audience,
    id_token: Option<String>,
) -> TokenIssue {
    TokenIssue {
        id_token,
        issued_token_type: None,
        refresh: match policy.refresh {
            RefreshTokenGrant::Granted => RefreshTokenIssue::Issued(random_token()),
            RefreshTokenGrant::Withheld => RefreshTokenIssue::Withheld,
        },
        resource,
        scopes,
        subject: subject.to_string(),
    }
}

struct TokenExchangeParameters {
    actor_token: Option<String>,
    audience: Option<String>,
    requested_token_type: Option<String>,
    resource: Option<String>,
    scope: Option<String>,
    subject_token: String,
    subject_token_type: String,
}

pub struct TokenEndpoint {
    clients: Arc<AcceptedClients>,
    exchangers: Arc<SubjectTokenExchangers>,
    issuance: Arc<dyn DeclaresTokenIssuance>,
    secret_store: Arc<JwksSecretStore>,
    state: Arc<dyn StoresProviderState>,
}

impl TokenEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        state: Arc<dyn StoresProviderState>,
        exchangers: Arc<SubjectTokenExchangers>,
        secret_store: Arc<JwksSecretStore>,
        issuance: Arc<dyn DeclaresTokenIssuance>,
    ) -> Self {
        Self {
            clients,
            exchangers,
            issuance,
            secret_store,
            state,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::State` when the provider state cannot be reached, and
    /// `ProviderError::Signing` when an id token cannot be signed.
    pub async fn respond(
        &self,
        request: &Request,
        token_request: ValidationResult<TokenRequest>,
    ) -> Result<Response, ProviderError> {
        let ValidationResult::Valid(token_request) = token_request else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token request is malformed",
            ));
        };
        let client = match authenticated_client(&self.clients, request, token_request.client_id()) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(client) => client,
        };
        let now = Utc::now();

        match token_request {
            TokenRequest::AuthorizationCode {
                code,
                code_verifier,
                redirect_uri,
                resource,
                ..
            } => {
                self.authorization_code(
                    client,
                    &code,
                    code_verifier,
                    &redirect_uri,
                    resource.as_deref(),
                    now,
                )
                .await
            }
            TokenRequest::ClientCredentials {
                resource, scope, ..
            } => Ok(self.client_credentials(client, resource.as_deref(), scope.as_deref(), now)),
            TokenRequest::RefreshToken {
                refresh_token,
                resource,
                scope,
                ..
            } => {
                self.refresh_token(
                    client,
                    &refresh_token,
                    resource.as_deref(),
                    scope.as_deref(),
                    now,
                )
                .await
            }
            TokenRequest::TokenExchange {
                actor_token,
                audience,
                requested_token_type,
                resource,
                scope,
                subject_token,
                subject_token_type,
                ..
            } => Ok(self
                .token_exchange(
                    client,
                    TokenExchangeParameters {
                        actor_token,
                        audience,
                        requested_token_type,
                        resource,
                        scope,
                        subject_token,
                        subject_token_type,
                    },
                    now,
                )
                .await),
            TokenRequest::GrantTypeOmitted => Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token request names no grant type",
            )),
            TokenRequest::Unsupported => Ok(oauth_error(
                400,
                BasicErrorResponseType::UnsupportedGrantType,
                "the grant type is not supported",
            )),
        }
    }

    async fn authorization_code(
        &self,
        client: &AcceptedClient,
        code: &str,
        code_verifier: String,
        redirect_uri: &str,
        resource: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        let AuthorizationCodeGrant::Granted(policy) = &client.authorization_code else {
            return Ok(unauthorized_client());
        };
        let Ok(redirect_uri) = Url::parse(redirect_uri) else {
            return Ok(invalid_grant("the redirect uri is not a url"));
        };
        let resource = match target_resource(&client.resources, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };
        let code = TokenDigest::of(code);
        let grant = match self
            .state
            .present_code(code)
            .await
            .map_err(ProviderError::State)?
        {
            PresentedCode::Issued(grant) => *grant,
            PresentedCode::Replayed => return Ok(replayed_code()),
            PresentedCode::Unknown => return Ok(unknown_code()),
        };
        let code_challenge =
            PkceCodeChallenge::from_code_verifier_sha256(&PkceCodeVerifier::new(code_verifier));

        if !(CodeAdmission {
            client_id: &client.client_id,
            code_challenge: code_challenge.as_str(),
            redirect_uri: &redirect_uri,
        })
        .admits(&grant)
        {
            return self
                .state
                .spend_code(code, RefreshIssuance::Withheld)
                .await
                .map_err(ProviderError::State)
                .map(|spending| {
                    code_settled(spending, || {
                        invalid_grant(
                            "the authorization code was granted to another client, callback or verifier",
                        )
                    })
                });
        }

        future::ready(self.id_token(client, policy, &grant, now))
            .and_then(|id_token| {
                self.spent_code(
                    code,
                    self.prepared_tokens(
                        client,
                        redeemed_issue(policy, grant, resource, id_token),
                        now,
                    ),
                )
            })
            .await
    }

    fn client_credentials(
        &self,
        client: &AcceptedClient,
        resource: Option<&str>,
        scope: Option<&str>,
        now: DateTime<Utc>,
    ) -> Response {
        let AcceptedClientAuthentication::ClientSecretBasic {
            privileges:
                ConfidentialPrivileges {
                    client_credentials: ClientCredentialsGrant::Granted { scopes: granted },
                    ..
                },
            ..
        } = &client.authentication
        else {
            return unauthorized_client();
        };
        let scopes = match RequestedScope::resolved(scope, granted) {
            ControlFlow::Break(refusal) => return refusal,
            ControlFlow::Continue(scopes) => scopes,
        };
        let resource = match target_resource(&client.resources, resource) {
            ControlFlow::Break(refusal) => return refusal,
            ControlFlow::Continue(resource) => resource,
        };

        self.prepared_tokens(
            client,
            TokenIssue {
                id_token: None,
                issued_token_type: None,
                refresh: RefreshTokenIssue::Withheld,
                resource,
                scopes,
                subject: client.client_id.as_str().to_string(),
            },
            now,
        )
        .response()
    }

    fn id_token(
        &self,
        client: &AcceptedClient,
        policy: &CodeGrantPolicy,
        AuthorizationGrant {
            auth_time,
            nonce,
            scopes,
            subject,
            ..
        }: &AuthorizationGrant,
        now: DateTime<Utc>,
    ) -> Result<Option<String>, ProviderError> {
        scopes
            .iter()
            .any(Scope::is_openid)
            .then(|| IdTokenClaims {
                auth_time: NumericDate::from(*auth_time),
                client_id: client.client_id.clone(),
                nonce: nonce.clone(),
                subject: *subject,
            })
            .map(|claims| {
                self.secret_store
                    .issue_id_token(&claims, policy.id_token_signing, now)
            })
            .transpose()
            .map_err(ProviderError::Signing)
    }

    fn prepared_tokens(
        &self,
        client: &AcceptedClient,
        TokenIssue {
            id_token,
            issued_token_type,
            refresh,
            resource,
            scopes,
            subject,
        }: TokenIssue,
        now: DateTime<Utc>,
    ) -> PreparedTokens {
        let audience = if scopes.iter().any(Scope::is_openid) {
            AudienceClaim::Multiple(vec![
                resource.as_str().to_string(),
                self.issuance.token_issuance().issuer.as_str().to_string(),
            ])
        } else {
            AudienceClaim::Single(resource.as_str().to_string())
        };
        let claims = ResourceAccessTokenClaims {
            client_id: client.client_id.clone(),
            scope: ScopeList { scopes },
            subject,
        };

        PreparedTokens {
            access_token: self.secret_store.issue_access_token(&claims, audience, now),
            fields: ProviderTokenFields {
                id_token,
                issued_token_type: issued_token_type.map(|token_type| token_type.urn().to_string()),
            },
            refresh,
            scope: claims.scope,
        }
    }

    async fn refresh_token(
        &self,
        client: &AcceptedClient,
        refresh_token: &str,
        resource: Option<&str>,
        scope: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        let AuthorizationCodeGrant::Granted(CodeGrantPolicy {
            refresh: RefreshTokenGrant::Granted,
            ..
        }) = &client.authorization_code
        else {
            return Ok(unauthorized_client());
        };
        let requested = match RequestedScope::of(scope) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(requested) => requested,
        };
        let resource = match target_resource(&client.resources, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };
        let presented = TokenDigest::of(refresh_token);
        let RefreshFamily {
            client_id,
            scopes: granted,
            subject,
            ..
        } = match self
            .state
            .present_refresh_token(presented)
            .await
            .map_err(ProviderError::State)?
        {
            PresentedRefreshToken::Current(family) => family,
            PresentedRefreshToken::Replayed => return Ok(replayed_refresh_token()),
            PresentedRefreshToken::Unknown => return Ok(unknown_refresh_token()),
        };

        if client_id != client.client_id {
            return Ok(unknown_refresh_token());
        }

        let scopes = match requested.within(&granted) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(scopes) => scopes,
        };
        let next = random_token();
        let next_digest = TokenDigest::of(&next);
        let prepared = self.prepared_tokens(
            client,
            TokenIssue {
                id_token: None,
                issued_token_type: None,
                refresh: RefreshTokenIssue::Issued(next),
                resource,
                scopes,
                subject: subject.to_string(),
            },
            now,
        );

        self.state
            .rotate_refresh_token(presented, next_digest)
            .await
            .map_err(ProviderError::State)
            .map(|rotation| refresh_rotated(rotation, prepared))
    }

    async fn spent_code(
        &self,
        code: TokenDigest,
        prepared: PreparedTokens,
    ) -> Result<Response, ProviderError> {
        self.state
            .spend_code(code, prepared.refresh.issuance())
            .await
            .map_err(ProviderError::State)
            .map(|spending| code_settled(spending, || prepared.response()))
    }

    async fn token_exchange(
        &self,
        client: &AcceptedClient,
        TokenExchangeParameters {
            actor_token,
            audience,
            requested_token_type,
            resource,
            scope,
            subject_token,
            subject_token_type,
        }: TokenExchangeParameters,
        now: DateTime<Utc>,
    ) -> Response {
        if client.token_exchange == TokenExchangeGrant::Withheld {
            return unauthorized_client();
        }

        let Ok(token_type) = subject_token_type.parse::<SubjectTokenType>() else {
            return oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the subject token type is not supported",
            );
        };

        if actor_token.is_some()
            || requested_token_type.is_some_and(|requested| {
                !requested
                    .parse::<SubjectTokenType>()
                    .is_ok_and(|requested| requested == SubjectTokenType::AccessToken)
            })
            || (audience.is_some() && resource.is_some())
        {
            return oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token exchange asks for delegation, another token type or two targets",
            );
        }

        let resource = match target_resource(&client.resources, audience.or(resource).as_deref()) {
            ControlFlow::Break(refusal) => return refusal,
            ControlFlow::Continue(resource) => resource,
        };

        match self
            .exchangers
            .exchange(&subject_token, token_type, now)
            .await
        {
            ExchangedSubject::Granted { scopes, subject } => {
                match RequestedScope::resolved(scope.as_deref(), &scopes) {
                    ControlFlow::Break(refusal) => refusal,
                    ControlFlow::Continue(scopes) => self
                        .prepared_tokens(
                            client,
                            TokenIssue {
                                id_token: None,
                                issued_token_type: Some(SubjectTokenType::AccessToken),
                                refresh: RefreshTokenIssue::Withheld,
                                resource,
                                scopes,
                                subject: subject.to_string(),
                            },
                            now,
                        )
                        .response(),
                }
            }
            ExchangedSubject::Refused(refusal) => refused_subject_token(&refusal),
            ExchangedSubject::SigningKeysAwaited => oauth_error(
                503,
                BasicErrorResponseType::Extension("temporarily_unavailable".to_string()),
                "the signing keys of the subject token issuer are not available yet",
            ),
        }
    }
}
