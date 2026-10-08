use std::future::ready;
use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use futures_util::TryFutureExt as _;
use oauth2::basic::BasicErrorResponseType;
use url::Url;
use uuid::Uuid;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::registered_authentication::RegisteredAuthentication;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_identity_session::id_token_claims::IdTokenClaims;
use margaret_identity_session::issued_access_token_claims::IssuedAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oauth_vocabulary::openid_scope::OPENID_SCOPE;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_oauth_vocabulary::subject_token_type_parsing::SubjectTokenTypeParsing;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;
use margaret_token_issuance::token_issuance::TokenIssuance;
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
use crate::temporarily_unavailable::temporarily_unavailable;
use crate::token_grant::TokenGrant;
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

async fn revoked_family(
    grants: &dyn StoresAuthorizationGrants,
    family: Uuid,
) -> Result<Response, ProviderError> {
    grants
        .revoke_refresh_family(family)
        .await
        .map_err(ProviderError::RevokeRefreshFamily)
        .map(|()| replayed_refresh_token())
}

async fn committed_tokens(
    grants: &dyn StoresAuthorizationGrants,
    family: Uuid,
    grant: &AuthorizationGrant,
    now: NumericDate,
    prepared: PreparedTokens,
) -> Result<Response, ProviderError> {
    match &prepared.refresh {
        RefreshTokenIssue::Issued(refresh_token) => grants
            .open_refresh_family(
                family,
                RefreshFamily::opened_by(grant, now),
                TokenDigest::of(refresh_token),
            )
            .await
            .map_err(ProviderError::OpenRefreshFamily)
            .map(|opening| match opening {
                FamilyOpening::Opened => prepared.response(),
                FamilyOpening::Revoked => replayed_code(),
            }),
        RefreshTokenIssue::Withheld => Ok(prepared.response()),
    }
}

fn redeemed_issue(
    policy: &CodeGrantPolicy,
    AuthorizationGrant {
        scopes, subject, ..
    }: &AuthorizationGrant,
    resource: &'static str,
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
        scopes: scopes
            .iter()
            .map(|scope| scope.as_str().to_string())
            .collect(),
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
    issuance: TokenIssuance,
    secret_store: Arc<JwksSecretStore>,
}

impl TokenEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        exchangers: Arc<SubjectTokenExchangers>,
        secret_store: Arc<JwksSecretStore>,
        issuance: TokenIssuance,
    ) -> Self {
        Self {
            clients,
            exchangers,
            issuance,
            secret_store,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::ClientAuthentication` when the client assertion cannot be
    /// remembered, a grant store variant when the application cannot reach its grants,
    /// `ProviderError::Signing` when an id token cannot be signed, and
    /// `ProviderError::SubjectTokenExchange` when a subject token exchanger fails.
    pub async fn respond(
        &self,
        request: &Request,
        token_request: ValidationResult<TokenRequest>,
    ) -> Result<Response, ProviderError> {
        let ValidationResult::Valid(TokenRequest {
            client_authentication,
            grant,
        }) = token_request
        else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token request is malformed",
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
        let client = &registered.client;

        match grant {
            TokenGrant::AuthorizationCode {
                code,
                code_verifier,
                redirect_uri,
                resource,
            } => {
                self.authorization_code(
                    registered,
                    &code,
                    code_verifier,
                    &redirect_uri,
                    resource.as_deref(),
                    now,
                )
                .await
            }
            TokenGrant::ClientCredentials { resource, scope } => {
                Ok(self.client_credentials(registered, resource.as_deref(), scope.as_deref(), now))
            }
            TokenGrant::RefreshToken {
                refresh_token,
                resource,
                scope,
            } => {
                self.refresh_token(
                    registered,
                    &refresh_token,
                    resource.as_deref(),
                    scope.as_deref(),
                    now,
                )
                .await
            }
            TokenGrant::TokenExchange {
                actor_token,
                audience,
                requested_token_type,
                resource,
                scope,
                subject_token,
                subject_token_type,
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
            TokenGrant::GrantTypeOmitted => Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token request names no grant type",
            )),
            TokenGrant::Unsupported => Ok(oauth_error(
                400,
                BasicErrorResponseType::UnsupportedGrantType,
                "the grant type is not supported",
            )),
        }
    }

    async fn authorization_code(
        &self,
        registered: &RegisteredClient,
        code: &str,
        code_verifier: String,
        redirect_uri: &str,
        resource: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        let RegisteredClient {
            client,
            code_grant: RegisteredCodeGrant::Granted { grants, policy, .. },
            ..
        } = registered
        else {
            return Ok(unauthorized_client());
        };
        let Ok(redirect_uri) = Url::parse(redirect_uri) else {
            return Ok(invalid_grant("the redirect uri is not a url"));
        };
        let resource = match target_resource(client.resources, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };
        let family = Uuid::new_v4();
        let instant = NumericDate::from(now);
        let grant = match grants
            .redeem_code(TokenDigest::of(code), family)
            .await
            .map_err(ProviderError::RedeemCode)?
        {
            CodeRedemption::Redeemed(issued) if issued.expires_at > instant => {
                let IssuedCode { grant, .. } = *issued;

                grant
            }
            CodeRedemption::Redeemed(_) | CodeRedemption::Unknown => return Ok(unknown_code()),
            CodeRedemption::AlreadyRedeemed { family } => {
                return grants
                    .revoke_refresh_family(family)
                    .await
                    .map_err(ProviderError::RevokeRefreshFamily)
                    .map(|()| replayed_code());
            }
        };

        if let ControlFlow::Break(refusal) = (CodeAdmission {
            client_id: client.client_id,
            code_verifier: &code_verifier,
            redirect_uri: &redirect_uri,
        })
        .admission(&grant)
        {
            return Ok(invalid_grant(refusal.description()));
        }

        ready(self.id_token(client, policy, &grant, now))
            .and_then(|id_token| {
                committed_tokens(
                    grants.as_ref(),
                    family,
                    &grant,
                    instant,
                    self.prepared_tokens(
                        client,
                        redeemed_issue(policy, &grant, resource, id_token),
                        now,
                    ),
                )
            })
            .await
    }

    fn client_credentials(
        &self,
        registered: &RegisteredClient,
        resource: Option<&str>,
        scope: Option<&str>,
        now: DateTime<Utc>,
    ) -> Response {
        let RegisteredClient {
            authentication:
                RegisteredAuthentication::PrivateKeyJwt {
                    privileges:
                        ConfidentialPrivileges {
                            client_credentials: ClientCredentialsGrant::Granted { scopes: granted },
                            ..
                        },
                    ..
                },
            client,
            ..
        } = registered
        else {
            return unauthorized_client();
        };
        let scopes = match RequestedScope::resolved(scope, &granted.iter().copied().collect()) {
            ControlFlow::Break(refusal) => return refusal,
            ControlFlow::Continue(scopes) => scopes,
        };
        let resource = match target_resource(client.resources, resource) {
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
                subject: client.client_id.to_string(),
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
                client_id: client.client_id,
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
        let audience = if scopes.contains(OPENID_SCOPE) {
            AudienceClaim::Multiple(vec![resource.to_string(), self.issuance.issuer.to_string()])
        } else {
            AudienceClaim::Single(resource.to_string())
        };
        let claims = IssuedAccessTokenClaims {
            client_id: client.client_id,
            scopes,
            subject,
        };

        PreparedTokens {
            access_token: self.secret_store.issue_access_token(&claims, audience, now),
            fields: ProviderTokenFields {
                id_token,
                issued_token_type: issued_token_type.map(|token_type| token_type.urn().to_string()),
            },
            refresh,
            scopes: claims.scopes,
        }
    }

    async fn refresh_token(
        &self,
        registered: &RegisteredClient,
        refresh_token: &str,
        resource: Option<&str>,
        scope: Option<&str>,
        now: DateTime<Utc>,
    ) -> Result<Response, ProviderError> {
        let RegisteredClient {
            client,
            code_grant:
                RegisteredCodeGrant::Granted {
                    grants,
                    policy:
                        CodeGrantPolicy {
                            refresh: RefreshTokenGrant::Granted,
                            ..
                        },
                    ..
                },
            ..
        } = registered
        else {
            return Ok(unauthorized_client());
        };
        let requested = match RequestedScope::of(scope) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(requested) => requested,
        };
        let resource = match target_resource(client.resources, resource) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };
        let presented = TokenDigest::of(refresh_token);
        let RefreshFamily {
            scopes: granted,
            subject,
            ..
        } = match grants
            .find_refresh_token(presented)
            .await
            .map_err(ProviderError::FindRefreshToken)?
        {
            RefreshTokenLookup::Current { record, .. }
                if record.expires_at > NumericDate::from(now)
                    && record.client_id == client.client_id =>
            {
                record
            }
            RefreshTokenLookup::Current { .. } | RefreshTokenLookup::Unknown => {
                return Ok(unknown_refresh_token());
            }
            RefreshTokenLookup::Superseded { family } => {
                return revoked_family(grants.as_ref(), family).await;
            }
        };
        let scopes = match requested.within(&granted.iter().map(Scope::as_str).collect()) {
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

        match grants
            .rotate_refresh_token(presented, next_digest)
            .await
            .map_err(ProviderError::RotateRefreshToken)?
        {
            RefreshRotation::Rotated => Ok(prepared.response()),
            RefreshRotation::Superseded { family } => revoked_family(grants.as_ref(), family).await,
            RefreshRotation::Unknown => Ok(unknown_refresh_token()),
        }
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
        let TokenExchangeGrant::Granted {
            scopes: exchangeable,
        } = client.token_exchange
        else {
            return Ok(unauthorized_client());
        };

        let SubjectTokenTypeParsing::Accepted(token_type) =
            SubjectTokenType::parse(&subject_token_type)
        else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the subject token type is not supported",
            ));
        };

        if actor_token.is_some()
            || requested_token_type.is_some_and(|requested| {
                SubjectTokenType::parse(&requested)
                    != SubjectTokenTypeParsing::Accepted(SubjectTokenType::AccessToken)
            })
            || (audience.is_some() && resource.is_some())
        {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the token exchange asks for delegation, another token type or two targets",
            ));
        }

        let resource = match target_resource(client.resources, audience.or(resource).as_deref()) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(resource) => resource,
        };

        Ok(
            match self
                .exchangers
                .exchange(&subject_token, token_type, now)
                .await
                .map_err(ProviderError::SubjectTokenExchange)?
            {
                ExchangedSubject::Granted { scopes, subject } => {
                    if let Some(undeclared) = scopes
                        .iter()
                        .find(|granted| !exchangeable.contains(&granted.as_str()))
                    {
                        return Err(ProviderError::UndeclaredExchangeScope {
                            client_id: client.client_id,
                            scope: undeclared.as_str().to_string(),
                        });
                    }

                    match RequestedScope::resolved(
                        scope.as_deref(),
                        &scopes.iter().map(Scope::as_str).collect(),
                    ) {
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
                ExchangedSubject::SigningKeysAwaited => temporarily_unavailable(
                    "the signing keys of the subject token issuer are not available yet",
                ),
            },
        )
    }
}
