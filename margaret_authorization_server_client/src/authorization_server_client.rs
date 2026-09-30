use std::borrow::Cow;
use std::sync::Arc;

use chrono::Utc;
use headers::Authorization;
use headers::HeaderMapExt;
use http::HeaderValue;
use http::StatusCode;
use http::header::ACCEPT;
use http::header::CONTENT_TYPE;
use oauth2::AccessToken;
use oauth2::AuthType;
use oauth2::AuthUrl;
use oauth2::AuthorizationCode;
use oauth2::Client;
use oauth2::ExtraTokenFields;
use oauth2::HttpResponse;
use oauth2::IntrospectionUrl;
use oauth2::PkceCodeVerifier;
use oauth2::RedirectUrl;
use oauth2::RequestTokenError;
use oauth2::StandardRevocableToken;
use oauth2::StandardTokenIntrospectionResponse;
use oauth2::StandardTokenResponse;
use oauth2::TokenIntrospectionResponse;
use oauth2::TokenResponse;
use oauth2::TokenUrl;
use oauth2::basic::BasicErrorResponse;
use oauth2::basic::BasicRevocationErrorResponse;
use oauth2::basic::BasicTokenIntrospectionResponse;
use oauth2::basic::BasicTokenResponse;
use oauth2::basic::BasicTokenType;
use reqwest::Method;
use reqwest::Request;
use serde::de::DeserializeOwned;
use url::Url;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::declares_oauth_client::DeclaresOAuthClient;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::authorization_request::AuthorizationRequest;
use crate::authorization_server_client_error::AuthorizationServerClientError;
use crate::authorization_url::AuthorizationUrl;
use crate::client_assertion_claims::ClientAssertionClaims;
use crate::endpoint_outcome::EndpointOutcome;
use crate::form_encoded::form_encoded;
use crate::form_parameter::FormParameter;
use crate::grant_credentials::GrantCredentials;
use crate::server_endpoint::ServerEndpoint;
use crate::server_unavailability::ServerUnavailability;
use crate::token_target::TokenTarget;
use crate::unavailable_outcome::UnavailableOutcome;
use crate::userinfo_outcome::UserinfoOutcome;

const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
const JSON_CONTENT_TYPE: &str = "application/json";
const JWT_BEARER_ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

fn grant_answer<TAnswer: DeserializeOwned>(
    answer: HttpResponse,
) -> Result<TAnswer, RequestTokenError<IssuerExchangeError, BasicErrorResponse>> {
    let answered = answer.status() == StatusCode::OK;
    let body = answer.into_body();

    if answered {
        match serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(&body)) {
            Ok(answer) => Ok(answer),
            Err(source) => Err(RequestTokenError::Parse(source, body)),
        }
    } else {
        Err(
            match serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(&body))
            {
                Ok(refusal) => RequestTokenError::ServerResponse(refusal),
                Err(source) => RequestTokenError::Parse(source, body),
            },
        )
    }
}

fn grant_request(token_url: Url, grant_type: &str, parameters: &[FormParameter]) -> Request {
    let mut request = Request::new(Method::POST, token_url);

    request
        .headers_mut()
        .insert(ACCEPT, HeaderValue::from_static(JSON_CONTENT_TYPE));
    request
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static(FORM_CONTENT_TYPE));
    *request.body_mut() = Some(form_body(grant_type, parameters).into());

    request
}

fn form_body(grant_type: &str, parameters: &[FormParameter]) -> Vec<u8> {
    let mut form = form_urlencoded::Serializer::new(String::new());

    form.append_pair("grant_type", grant_type);

    for FormParameter { name, value } in parameters {
        form.append_pair(name, value);
    }

    form.finish().into_bytes()
}

fn userinfo_answer<TUserinfo: DeserializeOwned>(
    answer: &HttpResponse,
) -> UserinfoOutcome<TUserinfo> {
    if answer.status() != StatusCode::OK {
        return UserinfoOutcome::Refused {
            status: answer.status(),
        };
    }

    match serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(answer.body()))
    {
        Ok(userinfo) => UserinfoOutcome::Answered(userinfo),
        Err(source) => {
            UserinfoOutcome::Unavailable(ServerUnavailability::MalformedAnswer { source })
        }
    }
}

pub struct AuthorizationServerClient {
    pub declaration: Arc<dyn DeclaresOAuthClient>,
    pub metadata: Arc<IssuerMetadata>,
    request_client: Arc<IssuerRequestClient>,
    pub trusted_issuer: Arc<TrustedIssuer>,
}

impl AuthorizationServerClient {
    #[must_use]
    pub fn create(
        request_client: Arc<IssuerRequestClient>,
        metadata: Arc<IssuerMetadata>,
        trusted_issuer: Arc<TrustedIssuer>,
        declaration: Arc<dyn DeclaresOAuthClient>,
    ) -> Self {
        Self {
            declaration,
            metadata,
            request_client,
            trusted_issuer,
        }
    }

    pub async fn authorization_url(
        &self,
        AuthorizationRequest {
            nonce,
            pkce_challenge,
            redirect_uri,
            scopes,
            state,
        }: AuthorizationRequest,
    ) -> AuthorizationUrl {
        self.at_endpoint(ServerEndpoint::Authorization, async |authorization_url| {
            let (url, _state) = self
                .server_client::<BasicTokenResponse, BasicTokenIntrospectionResponse>()
                .set_auth_uri(AuthUrl::from_url(authorization_url))
                .authorize_url(|| state)
                .add_scopes(scopes)
                .set_pkce_challenge(pkce_challenge)
                .set_redirect_uri(Cow::Owned(redirect_uri))
                .add_extra_param("nonce", nonce)
                .url();

            AuthorizationUrl::Built(url)
        })
        .await
    }

    /// # Errors
    ///
    /// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
    /// `private_key_jwt` client cannot be signed.
    pub async fn client_credentials(
        &self,
        target: &TokenTarget,
    ) -> Result<EndpointOutcome<BasicTokenResponse>, AuthorizationServerClientError> {
        self.at_endpoint(ServerEndpoint::Token, async |token_url| {
            let client = self
                .server_client::<BasicTokenResponse, BasicTokenIntrospectionResponse>()
                .set_token_uri(TokenUrl::from_url(token_url));
            let request = self
                .assertion_parameters()?
                .into_iter()
                .chain(target.form_parameters())
                .fold(
                    client.exchange_client_credentials(),
                    |request, FormParameter { name, value }| request.add_extra_param(name, value),
                );

            Ok(EndpointOutcome::of_bearer_token(
                request.request_async(self.request_client.as_ref()).await,
            ))
        })
        .await
    }

    /// # Errors
    ///
    /// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
    /// `private_key_jwt` client cannot be signed.
    pub async fn exchange_authorization_code<TExtraFields: ExtraTokenFields>(
        &self,
        code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
        redirect_uri: RedirectUrl,
    ) -> Result<
        EndpointOutcome<StandardTokenResponse<TExtraFields, BasicTokenType>>,
        AuthorizationServerClientError,
    > {
        self.at_endpoint(ServerEndpoint::Token, async |token_url| {
            let client = self
                .server_client::<
                    StandardTokenResponse<TExtraFields, BasicTokenType>,
                    BasicTokenIntrospectionResponse,
                >()
                .set_token_uri(TokenUrl::from_url(token_url));
            let request = self.assertion_parameters()?.into_iter().fold(
                client
                    .exchange_code(code)
                    .set_pkce_verifier(pkce_verifier)
                    .set_redirect_uri(Cow::Owned(redirect_uri)),
                |request, FormParameter { name, value }| request.add_extra_param(name, value),
            );

            Ok(EndpointOutcome::of_bearer_token(
                request.request_async(self.request_client.as_ref()).await,
            ))
        })
        .await
    }

    /// # Errors
    ///
    /// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
    /// `private_key_jwt` client cannot be signed.
    pub async fn introspect<TExtraFields: ExtraTokenFields>(
        &self,
        token: &str,
    ) -> Result<
        EndpointOutcome<StandardTokenIntrospectionResponse<TExtraFields, BasicTokenType>>,
        AuthorizationServerClientError,
    > {
        self.at_endpoint(ServerEndpoint::Introspection, async |introspection_url| {
            let client = self
                .server_client::<
                    BasicTokenResponse,
                    StandardTokenIntrospectionResponse<TExtraFields, BasicTokenType>,
                >()
                .set_introspection_url(IntrospectionUrl::from_url(introspection_url));
            let access_token = AccessToken::new(token.to_string());
            let request = self.assertion_parameters()?.into_iter().fold(
                client
                    .introspect(&access_token)
                    .set_token_type_hint("access_token"),
                |request, FormParameter { name, value }| request.add_extra_param(name, value),
            );

            Ok(EndpointOutcome::of(
                request.request_async(self.request_client.as_ref()).await,
            ))
        })
        .await
    }

    /// # Errors
    ///
    /// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
    /// `private_key_jwt` client cannot be signed.
    pub async fn request_grant<TExtraFields: ExtraTokenFields>(
        &self,
        grant_type: &str,
        parameters: Vec<FormParameter>,
    ) -> Result<
        EndpointOutcome<StandardTokenResponse<TExtraFields, BasicTokenType>>,
        AuthorizationServerClientError,
    > {
        self.at_endpoint(ServerEndpoint::Token, async |token_url| {
            let request = match self.grant_credentials()? {
                GrantCredentials::AuthorizationHeader(authorization) => {
                    let mut request = grant_request(token_url, grant_type, &parameters);

                    request.headers_mut().typed_insert(authorization);

                    request
                }
                GrantCredentials::BodyParameters(credentials) => grant_request(
                    token_url,
                    grant_type,
                    &parameters
                        .into_iter()
                        .chain(credentials)
                        .collect::<Vec<_>>(),
                ),
            };

            Ok(EndpointOutcome::of_bearer_token(
                match self.request_client.exchange(request).await {
                    Ok(answer) => grant_answer(answer),
                    Err(failure) => Err(RequestTokenError::Request(failure)),
                },
            ))
        })
        .await
    }

    pub async fn userinfo<TUserinfo: DeserializeOwned>(
        &self,
        access_token: &AccessToken,
    ) -> UserinfoOutcome<TUserinfo> {
        self.at_endpoint(ServerEndpoint::Userinfo, async |userinfo_url| {
            let authorization = match Authorization::bearer(access_token.secret()) {
                Ok(authorization) => authorization,
                Err(source) => {
                    return UserinfoOutcome::Unavailable(
                        ServerUnavailability::AccessTokenUnpresentable { source },
                    );
                }
            };
            let mut request = Request::new(Method::GET, userinfo_url);

            request
                .headers_mut()
                .insert(ACCEPT, HeaderValue::from_static(JSON_CONTENT_TYPE));
            request.headers_mut().typed_insert(authorization);

            match self.request_client.exchange(request).await {
                Ok(answer) => userinfo_answer(&answer),
                Err(failure) => UserinfoOutcome::Unavailable(ServerUnavailability::from(failure)),
            }
        })
        .await
    }

    fn assertion_parameters(&self) -> Result<Vec<FormParameter>, AuthorizationServerClientError> {
        let OAuthClient {
            authentication,
            client_id,
        } = self.declaration.oauth_client();

        match authentication {
            ClientAuthentication::ClientSecretBasic(_) => Ok(Vec::new()),
            ClientAuthentication::PrivateKeyJwt(signer) => signer
                .sign_client_assertion(&ClientAssertionClaims::new(
                    client_id,
                    &self.trusted_issuer.trust.token_trust().issuer,
                    Utc::now(),
                ))
                .map(|assertion| {
                    vec![
                        FormParameter {
                            name: "client_assertion",
                            value: assertion,
                        },
                        FormParameter {
                            name: "client_assertion_type",
                            value: JWT_BEARER_ASSERTION_TYPE.to_string(),
                        },
                    ]
                })
                .map_err(AuthorizationServerClientError::AssertionSigning),
        }
    }

    async fn at_endpoint<TOutcome: UnavailableOutcome>(
        &self,
        endpoint: ServerEndpoint,
        exchange: impl AsyncFnOnce(Url) -> TOutcome,
    ) -> TOutcome {
        match self.metadata.holding() {
            MetadataHolding::Awaiting => {
                TOutcome::unavailable(ServerUnavailability::MetadataAwaited)
            }
            MetadataHolding::Held(metadata) => match endpoint.advertised_in(&metadata) {
                AdvertisedEndpoint::Advertised(url) => exchange(url.clone()).await,
                AdvertisedEndpoint::Unadvertised => {
                    TOutcome::unavailable(ServerUnavailability::EndpointUnadvertised { endpoint })
                }
            },
        }
    }

    fn grant_credentials(&self) -> Result<GrantCredentials, AuthorizationServerClientError> {
        let OAuthClient {
            authentication,
            client_id,
        } = self.declaration.oauth_client();

        match authentication {
            ClientAuthentication::ClientSecretBasic(secret) => {
                Ok(GrantCredentials::AuthorizationHeader(Authorization::basic(
                    &form_encoded(client_id.as_str()),
                    &form_encoded(secret.expose()),
                )))
            }
            ClientAuthentication::PrivateKeyJwt(_) => {
                self.assertion_parameters().map(|mut parameters| {
                    parameters.push(FormParameter {
                        name: "client_id",
                        value: client_id.as_str().to_string(),
                    });

                    GrantCredentials::BodyParameters(parameters)
                })
            }
        }
    }

    fn server_client<TTokenResponse: TokenResponse, TIntrospection: TokenIntrospectionResponse>(
        &self,
    ) -> Client<
        BasicErrorResponse,
        TTokenResponse,
        TIntrospection,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
    > {
        let OAuthClient {
            authentication,
            client_id,
        } = self.declaration.oauth_client();
        let client = Client::new(oauth2::ClientId::new(client_id.as_str().to_string()));

        match authentication {
            ClientAuthentication::ClientSecretBasic(secret) => {
                client.set_client_secret(oauth2::ClientSecret::new(secret.expose().to_string()))
            }
            ClientAuthentication::PrivateKeyJwt(_) => client.set_auth_type(AuthType::RequestBody),
        }
    }
}
