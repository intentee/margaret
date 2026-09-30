use std::collections::BTreeSet;
use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;
use oauth2::basic::BasicErrorResponseType;
use url::Url;
use uuid::Uuid;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_identity_session::id_token_claims::IdTokenClaims;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store::provider_identity::ProviderIdentity;
use margaret_jwks_secret_store::provider_tokens::ProviderTokens;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_provider_state_storage::code_admission::CodeAdmission;
use margaret_provider_state_storage::code_redemption::CodeRedemption;
use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::refresh_scope::RefreshScope;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage::token_digest::TokenDigest;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_token_exchange_client::subject_token_type::SubjectTokenType;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret_validation::validation_result::ValidationResult;

use crate::authenticated_client::authenticated_client;
use crate::includes_openid::includes_openid;
use crate::oauth_error::oauth_error;
use crate::provider_error::ProviderError;
use crate::provider_token_fields::ProviderTokenFields;
use crate::random_token::random_token;
use crate::target_resource::target_resource;
use crate::token_issue::TokenIssue;
use crate::token_request::TokenRequest;
use crate::token_response::token_response;

fn invalid_grant(description: &str) -> Response {
    oauth_error(400, BasicErrorResponseType::InvalidGrant, description)
}

fn invalid_scope() -> Response {
    oauth_error(
        400,
        BasicErrorResponseType::InvalidScope,
        "the requested scope exceeds the granted scope",
    )
}

fn narrowed_scopes(
    granted: &BTreeSet<Scope>,
    requested: Option<&str>,
) -> ControlFlow<Response, BTreeSet<Scope>> {
    match requested.map(str::parse::<ScopeList>) {
        Some(Ok(ScopeList { scopes })) if scopes.is_subset(granted) => {
            ControlFlow::Continue(scopes)
        }
        Some(Ok(_) | Err(_)) => ControlFlow::Break(invalid_scope()),
        None => ControlFlow::Continue(granted.clone()),
    }
}

fn unauthorized_client() -> Response {
    oauth_error(
        400,
        BasicErrorResponseType::UnauthorizedClient,
        "the client may not use the grant type",
    )
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
    /// `ProviderError::Signing` when a token cannot be signed.
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
            } => self.client_credentials(client, resource.as_deref(), scope.as_deref(), now),
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
            } => {
                self.token_exchange(
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
                .await
            }
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
        if !client.grants.contains(&GrantType::AuthorizationCode) {
            return Ok(unauthorized_client());
        }

        let Ok(redirect_uri) = Url::parse(redirect_uri) else {
            return Ok(invalid_grant("the redirect uri is not a url"));
        };
        let resource = match target_resource(client, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };
        let code_challenge =
            PkceCodeChallenge::from_code_verifier_sha256(&PkceCodeVerifier::new(code_verifier));
        let refresh_token = client
            .grants
            .contains(&GrantType::RefreshToken)
            .then(random_token);
        let refresh = match &refresh_token {
            Some(refresh_token) => RefreshIssuance::Opened(TokenDigest::of(refresh_token)),
            None => RefreshIssuance::Withheld,
        };

        self.state
            .redeem_code(
                TokenDigest::of(code),
                CodeRedemptionRequest {
                    admission: CodeAdmission {
                        client_id: &client.client_id,
                        code_challenge: code_challenge.as_str(),
                        redirect_uri: &redirect_uri,
                    },
                    family: Uuid::new_v4(),
                    refresh,
                },
            )
            .await
            .map_err(ProviderError::State)
            .and_then(|redemption| match redemption {
                CodeRedemption::Redeemed(grant) => {
                    let AuthorizationGrant {
                        auth_time,
                        nonce,
                        scopes,
                        subject,
                        ..
                    } = *grant;

                    self.issued(
                        client,
                        TokenIssue {
                            identity: if includes_openid(&scopes) {
                                ProviderIdentity::Asserted {
                                    claims: IdTokenClaims {
                                        auth_time: NumericDate::from(auth_time),
                                        client_id: client.client_id.clone(),
                                        nonce,
                                        subject,
                                    },
                                    signing: client.id_token_signing,
                                }
                            } else {
                                ProviderIdentity::Withheld
                            },
                            issued_token_type: None,
                            refresh_token,
                            resource,
                            scopes,
                            subject: subject.to_string(),
                        },
                        now,
                    )
                }
                CodeRedemption::Refused => Ok(invalid_grant(
                    "the authorization code was granted to another client, callback or verifier",
                )),
                CodeRedemption::Replayed => Ok(invalid_grant(
                    "the authorization code was already redeemed, so its tokens are revoked",
                )),
                CodeRedemption::Unknown => Ok(invalid_grant("the authorization code is not known")),
            })
    }

    fn client_credentials(
        &self,
        client: &AcceptedClient,
        resource: Option<&str>,
        scope: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        if !client.grants.contains(&GrantType::ClientCredentials) {
            return Ok(unauthorized_client());
        }

        let scopes = match narrowed_scopes(&client.scopes, scope) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(scopes) => scopes,
        };
        let resource = match target_resource(client, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };

        self.issued(
            client,
            TokenIssue {
                identity: ProviderIdentity::Withheld,
                issued_token_type: None,
                refresh_token: None,
                resource,
                scopes,
                subject: client.client_id.as_str().to_string(),
            },
            now,
        )
    }

    fn issued(
        &self,
        client: &AcceptedClient,
        TokenIssue {
            identity,
            issued_token_type,
            refresh_token,
            resource,
            scopes,
            subject,
        }: TokenIssue,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        let audience = if includes_openid(&scopes) {
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

        self.secret_store
            .issue_provider_tokens(&claims, audience, identity, now)
            .map_err(ProviderError::Signing)
            .map(
                |ProviderTokens {
                     access_token,
                     id_token,
                 }| {
                    token_response(
                        access_token,
                        &claims.scope,
                        refresh_token,
                        ProviderTokenFields {
                            id_token,
                            issued_token_type,
                        },
                    )
                },
            )
    }

    async fn refresh_token(
        &self,
        client: &AcceptedClient,
        refresh_token: &str,
        resource: Option<&str>,
        scope: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        if !client.grants.contains(&GrantType::RefreshToken) {
            return Ok(unauthorized_client());
        }

        let refresh_scope = match scope.map(str::parse::<ScopeList>) {
            Some(Ok(ScopeList { scopes })) => RefreshScope::Narrowed(scopes),
            Some(Err(_)) => return Ok(invalid_scope()),
            None => RefreshScope::Granted,
        };
        let resource = match target_resource(client, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };
        let next = random_token();

        self.state
            .rotate_refresh_token(
                TokenDigest::of(refresh_token),
                TokenDigest::of(&next),
                RefreshAdmission {
                    client_id: &client.client_id,
                    scope: &refresh_scope,
                },
            )
            .await
            .map_err(ProviderError::State)
            .and_then(|rotation| match rotation {
                RefreshRotation::ForeignClient | RefreshRotation::Unknown => {
                    Ok(invalid_grant("the refresh token is not known"))
                }
                RefreshRotation::Replayed => Ok(invalid_grant(
                    "the refresh token was already rotated, so its family is revoked",
                )),
                RefreshRotation::Rotated(RefreshFamily {
                    scopes: granted,
                    subject,
                    ..
                }) => self.issued(
                    client,
                    TokenIssue {
                        identity: ProviderIdentity::Withheld,
                        issued_token_type: None,
                        refresh_token: Some(next),
                        resource,
                        scopes: match refresh_scope {
                            RefreshScope::Granted => granted,
                            RefreshScope::Narrowed(scopes) => scopes,
                        },
                        subject: subject.to_string(),
                    },
                    now,
                ),
                RefreshRotation::ScopeExceeded => Ok(invalid_scope()),
            })
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
    ) -> Result<Response, ProviderError> {
        if !client.grants.contains(&GrantType::TokenExchange) {
            return Ok(unauthorized_client());
        }

        let Some(token_type) = SubjectTokenType::ALL
            .into_iter()
            .find(|token_type| token_type.urn() == subject_token_type)
        else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the subject token type is not supported",
            ));
        };

        if actor_token.is_some()
            || requested_token_type
                .is_some_and(|requested| requested != SubjectTokenType::AccessToken.urn())
            || (audience.is_some() && resource.is_some())
        {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token exchange asks for delegation, another token type or two targets",
            ));
        }

        let resource = match target_resource(client, audience.or(resource).as_deref()) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };

        match self
            .exchangers
            .exchange(&subject_token, token_type, now)
            .await
        {
            ExchangedSubject::Granted { scopes, subject } => {
                match narrowed_scopes(&scopes, scope.as_deref()) {
                    ControlFlow::Break(refusal) => Ok(refusal),
                    ControlFlow::Continue(scopes) => self.issued(
                        client,
                        TokenIssue {
                            identity: ProviderIdentity::Withheld,
                            issued_token_type: Some(
                                SubjectTokenType::AccessToken.urn().to_string(),
                            ),
                            refresh_token: None,
                            resource,
                            scopes,
                            subject: subject.to_string(),
                        },
                        now,
                    ),
                }
            }
            ExchangedSubject::Refused(refusal) => Ok(invalid_grant(&refusal.to_string())),
            ExchangedSubject::SigningKeysAwaited => Ok(oauth_error(
                503,
                BasicErrorResponseType::Extension("temporarily_unavailable".to_string()),
                "the signing keys of the subject token issuer are not available yet",
            )),
        }
    }
}
