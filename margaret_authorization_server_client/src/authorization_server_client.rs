use std::borrow::Cow;
use std::iter;
use std::sync::Arc;

use chrono::Utc;
use headers::Authorization;
use headers::HeaderMapExt;
use http::HeaderValue;
use http::Response;
use http::StatusCode;
use http::header::ACCEPT;
use http::header::CONTENT_TYPE;
use oauth2::AccessToken;
use oauth2::AuthUrl;
use oauth2::AuthorizationCode;
use oauth2::ExtraTokenFields;
use oauth2::RedirectUrl;
use oauth2::StandardTokenIntrospectionResponse;
use oauth2::StandardTokenResponse;
use oauth2::basic::BasicClient;
use oauth2::basic::BasicTokenResponse;
use oauth2::basic::BasicTokenType;
use reqwest::Method;
use reqwest::Request;
use serde::de::DeserializeOwned;
use url::Url;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_issuer_request::issuer_answer::IssuerAnswer;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_issuer_request::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::presented_client_authentication::PresentedClientAuthentication;
use margaret_oauth_vocabulary::client_secret::ClientSecret;
use margaret_oauth_vocabulary::code_challenge_method::CodeChallengeMethod;
use margaret_oauth_vocabulary::code_verifier::CodeVerifier;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_oauth_vocabulary::token_type_hint::TokenTypeHint;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::answer_parsing::AnswerParsing;
use crate::authorization_request::AuthorizationRequest;
use crate::authorization_url::AuthorizationUrl;
use crate::endpoint_outcome::EndpointOutcome;
use crate::form_parameter::FormParameter;
use crate::server_endpoint::ServerEndpoint;
use crate::server_unavailability::ServerUnavailability;
use crate::token_target::TokenTarget;
use crate::unavailable_outcome::UnavailableOutcome;
use crate::userinfo_outcome::UserinfoOutcome;

const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
const JSON_CONTENT_TYPE: &str = "application/json";

fn form_body(parameters: impl IntoIterator<Item = FormParameter>) -> Vec<u8> {
    let mut form = form_urlencoded::Serializer::new(String::new());

    for FormParameter { name, value } in parameters {
        form.append_pair(name, &value);
    }

    form.finish().into_bytes()
}

fn userinfo_answer<TUserinfo: DeserializeOwned>(
    answer: &Response<Vec<u8>>,
) -> UserinfoOutcome<TUserinfo> {
    if answer.status() != StatusCode::OK {
        return UserinfoOutcome::Refused {
            status: answer.status(),
        };
    }

    match AnswerParsing::of(answer.body()) {
        AnswerParsing::Parsed(userinfo) => UserinfoOutcome::Answered(userinfo),
        AnswerParsing::Malformed(source) => {
            UserinfoOutcome::Unavailable(ServerUnavailability::MalformedAnswer { source })
        }
    }
}

pub struct AuthorizationServerClient {
    authentication: ClientAuthentication,
    pub client_id: &'static str,
    pub metadata: Arc<IssuerMetadata>,
    request_client: Arc<IssuerRequestClient>,
    pub trusted_issuer: Arc<TrustedIssuer>,
}

impl AuthorizationServerClient {
    #[must_use]
    pub fn with_client_secret_basic(
        request_client: Arc<IssuerRequestClient>,
        metadata: Arc<IssuerMetadata>,
        trusted_issuer: Arc<TrustedIssuer>,
        client_id: &'static str,
        client_secret: ClientSecret,
    ) -> Self {
        Self {
            authentication: ClientAuthentication::ClientSecretBasic(client_secret),
            client_id,
            metadata,
            request_client,
            trusted_issuer,
        }
    }

    #[must_use]
    pub fn with_private_key_jwt(
        request_client: Arc<IssuerRequestClient>,
        metadata: Arc<IssuerMetadata>,
        trusted_issuer: Arc<TrustedIssuer>,
        client_id: &'static str,
        secrets: Arc<JwksSecretHolder>,
    ) -> Self {
        Self {
            authentication: ClientAuthentication::PrivateKeyJwt(secrets),
            client_id,
            metadata,
            request_client,
            trusted_issuer,
        }
    }

    pub async fn authorization_url(
        &self,
        AuthorizationRequest {
            code_challenge,
            nonce,
            redirect_uri,
            scopes,
            state,
        }: AuthorizationRequest,
    ) -> AuthorizationUrl {
        self.at_endpoint(ServerEndpoint::Authorization, async |authorization_url| {
            let (url, _state) = BasicClient::new(oauth2::ClientId::new(self.client_id.to_string()))
                .set_auth_uri(AuthUrl::from_url(authorization_url))
                .authorize_url(|| state)
                .add_scopes(scopes)
                .set_redirect_uri(Cow::Owned(redirect_uri))
                .add_extra_param("code_challenge", code_challenge.wire())
                .add_extra_param(
                    "code_challenge_method",
                    CodeChallengeMethod::S256.wire_name(),
                )
                .add_extra_param("nonce", nonce)
                .url();

            AuthorizationUrl::Built(url)
        })
        .await
    }

    pub async fn client_credentials(
        &self,
        target: &TokenTarget,
    ) -> EndpointOutcome<BasicTokenResponse> {
        self.request_grant(GrantType::ClientCredentials, target.form_parameters())
            .await
    }

    pub async fn exchange_authorization_code<TExtraFields: ExtraTokenFields>(
        &self,
        code: AuthorizationCode,
        code_verifier: &CodeVerifier,
        redirect_uri: RedirectUrl,
    ) -> EndpointOutcome<StandardTokenResponse<TExtraFields, BasicTokenType>> {
        self.request_grant(
            GrantType::AuthorizationCode,
            vec![
                FormParameter {
                    name: "code",
                    value: code.into_secret(),
                },
                FormParameter {
                    name: "code_verifier",
                    value: code_verifier.secret().to_string(),
                },
                FormParameter {
                    name: "redirect_uri",
                    value: redirect_uri.to_string(),
                },
            ],
        )
        .await
    }

    pub async fn introspect<TExtraFields: ExtraTokenFields>(
        &self,
        token: &str,
    ) -> EndpointOutcome<StandardTokenIntrospectionResponse<TExtraFields, BasicTokenType>> {
        self.posted_form(
            ServerEndpoint::Introspection,
            vec![
                FormParameter {
                    name: "token",
                    value: token.to_string(),
                },
                FormParameter {
                    name: "token_type_hint",
                    value: TokenTypeHint::AccessToken.wire_name().to_string(),
                },
            ],
        )
        .await
    }

    pub async fn request_grant<TExtraFields: ExtraTokenFields>(
        &self,
        grant_type: GrantType,
        parameters: Vec<FormParameter>,
    ) -> EndpointOutcome<StandardTokenResponse<TExtraFields, BasicTokenType>> {
        self.posted_form(
            ServerEndpoint::Token,
            iter::once(FormParameter {
                name: "grant_type",
                value: grant_type.wire_name().to_string(),
            })
            .chain(parameters)
            .collect(),
        )
        .await
        .bearer_checked()
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
                Ok(IssuerAnswer::Received(answer)) => userinfo_answer(&answer),
                Ok(IssuerAnswer::Oversized { max_bytes }) => {
                    UserinfoOutcome::Unavailable(ServerUnavailability::OversizedAnswer {
                        max_bytes,
                    })
                }
                Err(failure) => {
                    UserinfoOutcome::Unavailable(ServerUnavailability::Exchange(failure))
                }
            }
        })
        .await
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

    async fn posted_form<TAnswer: DeserializeOwned>(
        &self,
        endpoint: ServerEndpoint,
        parameters: Vec<FormParameter>,
    ) -> EndpointOutcome<TAnswer> {
        self.at_endpoint(endpoint, async |endpoint_url| {
            let issued_at = NumericDate::from(Utc::now());
            let mut request = Request::new(Method::POST, endpoint_url);
            let authentication_parameters = match self.authentication.presented_to(
                self.trusted_issuer.trust.issuer,
                self.client_id,
                issued_at,
                issued_at.after(ISSUER_REQUEST_TIMEOUT),
            ) {
                PresentedClientAuthentication::Basic(authorization) => {
                    request.headers_mut().typed_insert(authorization);

                    Vec::new()
                }
                PresentedClientAuthentication::Assertion(assertion) => {
                    vec![
                        FormParameter {
                            name: "client_assertion",
                            value: assertion,
                        },
                        FormParameter {
                            name: "client_assertion_type",
                            value: JWT_BEARER_CLIENT_ASSERTION_TYPE.to_string(),
                        },
                        FormParameter {
                            name: "client_id",
                            value: self.client_id.to_string(),
                        },
                    ]
                }
            };

            request
                .headers_mut()
                .insert(ACCEPT, HeaderValue::from_static(JSON_CONTENT_TYPE));
            request
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static(FORM_CONTENT_TYPE));
            *request.body_mut() =
                Some(form_body(parameters.into_iter().chain(authentication_parameters)).into());

            EndpointOutcome::of(self.request_client.exchange(request).await)
        })
        .await
    }
}
