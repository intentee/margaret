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
use oauth2::PkceCodeVerifier;
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
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_issuer_request::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use margaret_oauth_client::declares_oauth_client::DeclaresOAuthClient;
use margaret_oauth_client::presented_client_authentication::PresentedClientAuthentication;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

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
const JWT_BEARER_ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

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
            let (url, _state) = BasicClient::new(oauth2::ClientId::new(
                self.declaration
                    .oauth_client()
                    .client_id
                    .as_str()
                    .to_string(),
            ))
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
        pkce_verifier: PkceCodeVerifier,
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
                    value: pkce_verifier.into_secret(),
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
                    value: "access_token".to_string(),
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
                Ok(answer) => userinfo_answer(&answer),
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
            let client = self.declaration.oauth_client();
            let issued_at = NumericDate::from(Utc::now());
            let mut request = Request::new(Method::POST, endpoint_url);
            let authentication_parameters = match client.presented_to(
                &self.trusted_issuer.trust.token_trust().issuer,
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
                            value: JWT_BEARER_ASSERTION_TYPE.to_string(),
                        },
                        FormParameter {
                            name: "client_id",
                            value: client.client_id.as_str().to_string(),
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
