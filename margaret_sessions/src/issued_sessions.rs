use std::sync::Arc;

use chrono::DateTime;
use chrono::SubsecRound as _;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::database::database::Database;
use margaret_http::cookie_changes::CookieChanges;
use margaret_http::request::Request;
use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;
use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;

use crate::cookie_domain::CookieDomain;
use crate::resolved_session::ResolvedSession;
use crate::session::Session;
use crate::session_cookie_jar::SessionCookieJar;
use crate::session_record::SessionRecord;
use crate::session_refresh::SessionRefresh;
use crate::sessions_error::SessionsError;
use crate::started_session::StartedSession;

pub struct IssuedSessions {
    audience: &'static str,
    cookies: SessionCookieJar,
    database: Arc<Database>,
    secret_store: Arc<JwksSecretStore>,
}

impl IssuedSessions {
    #[must_use]
    pub fn host_only(
        database: Arc<Database>,
        secret_store: Arc<JwksSecretStore>,
        audience: &'static str,
    ) -> Self {
        Self {
            audience,
            cookies: SessionCookieJar::HostOnly,
            database,
            secret_store,
        }
    }

    #[must_use]
    pub fn shared_with_domain(
        database: Arc<Database>,
        secret_store: Arc<JwksSecretStore>,
        audience: &'static str,
        domain: CookieDomain,
    ) -> Self {
        Self {
            audience,
            cookies: SessionCookieJar::SharedWithDomain(domain),
            database,
            secret_store,
        }
    }

    pub(crate) async fn refresh(&self, secret: &str) -> Result<SessionRefresh, SessionsError> {
        let now = Utc::now();

        Ok(
            match SessionRecord::current(
                &self.database,
                TokenDigest::of(secret),
                NumericDate::from(now),
            )
            .await?
            {
                Lookup::Found(session) => {
                    SessionRefresh::Refreshed(self.access_token(session, now))
                }
                Lookup::Missing => SessionRefresh::Refused,
            },
        )
    }

    /// # Errors
    ///
    /// Returns `SessionsError` when the session of a presented secret cannot be looked up.
    pub async fn resolve(&self, request: &Request) -> Result<ResolvedSession, SessionsError> {
        let now = Utc::now();
        let presented_access_token = self.cookies.presented_access_token(request);

        if let Some(access_token) = presented_access_token
            && let JwtVerification::Verified(verified) = self
                .secret_store
                .verify_session_access_token(access_token, self.audience, now)
        {
            return Ok(ResolvedSession {
                cookie_changes: CookieChanges {
                    cookies: Vec::new(),
                },
                session: Some(Session::from(verified.claims)),
            });
        }

        let Some(secret) = self.cookies.presented_secret(request) else {
            return Ok(ResolvedSession {
                cookie_changes: match presented_access_token {
                    Some(_) => self.cookies.removal(),
                    None => CookieChanges {
                        cookies: Vec::new(),
                    },
                },
                session: None,
            });
        };

        Ok(
            match SessionRecord::current(
                &self.database,
                TokenDigest::of(secret),
                NumericDate::from(now),
            )
            .await?
            {
                Lookup::Found(session) => ResolvedSession {
                    cookie_changes: CookieChanges {
                        cookies: vec![
                            self.cookies
                                .access_cookie(self.access_token(session, now).signed_claims),
                        ],
                    },
                    session: Some(session),
                },
                Lookup::Missing => ResolvedSession {
                    cookie_changes: self.cookies.removal(),
                    session: None,
                },
            },
        )
    }

    /// # Errors
    ///
    /// Returns `SessionsError` when the session of a presented secret cannot be forgotten.
    pub async fn sign_out(&self, request: &Request) -> Result<CookieChanges, SessionsError> {
        if let Some(secret) = self.cookies.presented_secret(request) {
            SessionRecord::forget(&self.database, TokenDigest::of(secret)).await?;
        }

        Ok(self.cookies.removal())
    }

    /// # Errors
    ///
    /// Returns `SessionsError` when the expired sessions cannot be swept or the session cannot be
    /// stored.
    pub async fn start(
        &self,
        subject: Uuid,
        authenticated_at: DateTime<Utc>,
    ) -> Result<StartedSession, SessionsError> {
        let now = Utc::now();
        let secret = random_token();
        let session = Session {
            authenticated_at: authenticated_at.trunc_subsecs(0),
            id: Uuid::now_v7(),
            subject,
        };

        SessionRecord::open(
            &self.database,
            TokenDigest::of(&secret),
            session,
            NumericDate::from(now),
        )
        .await?;

        Ok(StartedSession {
            cookie_changes: CookieChanges {
                cookies: vec![
                    self.cookies.secret_cookie(secret),
                    self.cookies
                        .access_cookie(self.access_token(session, now).signed_claims),
                ],
            },
            session,
        })
    }

    fn access_token(
        &self,
        Session {
            authenticated_at,
            id,
            subject,
        }: Session,
        now: DateTime<Utc>,
    ) -> AccessTokenClaimsSigned {
        self.secret_store.issue_session_access_token(
            &SessionAccessTokenClaims {
                auth_time: authenticated_at,
                sid: id,
                sub: subject,
            },
            self.audience,
            now,
        )
    }
}
