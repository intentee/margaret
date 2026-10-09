use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use http::HeaderValue;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;
use url::form_urlencoded::Serializer;

use margaret_http::cookie_changes::CookieChanges;
use margaret_http::request::Request;
use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_issuer_request::issuer_answer::IssuerAnswer;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::jwt_addressee::JwtAddressee as _;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::cookie_domain::CookieDomain;
use crate::resolved_session::ResolvedSession;
use crate::session::Session;
use crate::session_cookie_jar::SessionCookieJar;
use crate::session_refresh_answer::SessionRefreshAnswer;
use crate::session_refresh_url::SessionRefreshUrl;
use crate::session_resolution::SessionResolution;
use crate::session_unavailability::SessionUnavailability;
use crate::session_verification::SessionVerification;

pub struct ConsumedSessions {
    cookies: SessionCookieJar,
    issuer_requests: IssuerRequestClient,
    refresh_url: SessionRefreshUrl,
    trusted_issuer: Arc<TrustedIssuer>,
}

impl ConsumedSessions {
    #[must_use]
    pub fn create(
        trusted_issuer: Arc<TrustedIssuer>,
        domain: CookieDomain,
        refresh_url: SessionRefreshUrl,
        spiffe_http_client: Client,
    ) -> Self {
        Self {
            cookies: SessionCookieJar::SharedWithDomain(domain),
            issuer_requests: IssuerRequestClient::preconfigured(spiffe_http_client),
            refresh_url,
            trusted_issuer,
        }
    }

    pub async fn resolve(&self, request: &Request) -> SessionResolution {
        let now = NumericDate::from(Utc::now());
        let presented_access_token = self.cookies.presented_access_token(request);

        if let Some(access_token) = presented_access_token {
            match self.verified(access_token, now).await {
                SessionVerification::KeysAwaited => {
                    return SessionResolution::Unavailable(SessionUnavailability::KeysAwaited);
                }
                SessionVerification::Rejected(_) => {}
                SessionVerification::Verified(session) => {
                    return SessionResolution::Resolved(ResolvedSession {
                        cookie_changes: CookieChanges {
                            cookies: Vec::new(),
                        },
                        session: Some(session),
                    });
                }
            }
        }

        match self.cookies.presented_secret(request) {
            Some(secret) => self.refreshed(secret, now).await,
            None => SessionResolution::Resolved(ResolvedSession {
                cookie_changes: match presented_access_token {
                    Some(_) => self.cookies.removal(),
                    None => CookieChanges {
                        cookies: Vec::new(),
                    },
                },
                session: None,
            }),
        }
    }

    async fn refreshed(&self, secret: &str, now: NumericDate) -> SessionResolution {
        let mut refresh = reqwest::Request::new(Method::POST, self.refresh_url.url().clone());

        refresh.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        *refresh.body_mut() = Some(
            Serializer::new(String::new())
                .append_pair("secret", secret)
                .finish()
                .into(),
        );

        let answer = match self.issuer_requests.exchange(refresh).await {
            Ok(IssuerAnswer::Received(answer)) => answer,
            Ok(IssuerAnswer::Oversized { max_bytes }) => {
                return SessionResolution::Unavailable(
                    SessionUnavailability::OversizedRefreshAnswer { max_bytes },
                );
            }
            Err(failure) => {
                return SessionResolution::Unavailable(SessionUnavailability::RefreshExchange(
                    failure,
                ));
            }
        };

        match answer.status() {
            StatusCode::OK => match serde_json::from_slice::<SessionRefreshAnswer>(answer.body()) {
                Ok(SessionRefreshAnswer { access_token }) => {
                    match self.verified(&access_token, now).await {
                        SessionVerification::KeysAwaited => {
                            SessionResolution::Unavailable(SessionUnavailability::KeysAwaited)
                        }
                        SessionVerification::Rejected(rejection) => SessionResolution::Unavailable(
                            SessionUnavailability::UnverifiableRefreshedToken(rejection),
                        ),
                        SessionVerification::Verified(session) => {
                            SessionResolution::Resolved(ResolvedSession {
                                cookie_changes: CookieChanges {
                                    cookies: vec![self.cookies.access_cookie(access_token)],
                                },
                                session: Some(session),
                            })
                        }
                    }
                }
                Err(source) => {
                    SessionResolution::Unavailable(SessionUnavailability::MalformedRefreshAnswer {
                        source,
                    })
                }
            },
            StatusCode::UNAUTHORIZED => SessionResolution::Resolved(ResolvedSession {
                cookie_changes: self.cookies.removal(),
                session: None,
            }),
            status => {
                SessionResolution::Unavailable(SessionUnavailability::RefreshStatus { status })
            }
        }
    }

    async fn verified(&self, access_token: &str, now: NumericDate) -> SessionVerification {
        let attributed =
            match attribute_serialized_jwt(access_token, &self.trusted_issuer.jwt_expectation()) {
                ControlFlow::Continue(attributed) => attributed,
                ControlFlow::Break(rejection) => return SessionVerification::Rejected(rejection),
            };

        match self
            .trusted_issuer
            .verify::<SessionAccessTokenClaims, AccessTokenProfile>(&attributed, now)
            .await
        {
            IssuerVerification::KeysAwaited => SessionVerification::KeysAwaited,
            IssuerVerification::Rejected(rejection) => SessionVerification::Rejected(rejection),
            IssuerVerification::Verified(verified) => {
                SessionVerification::Verified(Session::from(verified.claims))
            }
        }
    }
}
