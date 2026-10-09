use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use cookie::Cookie;
use uuid::Uuid;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::cookie_changes::CookieChanges;
use margaret_http::head_handler::HeadHandler;
use margaret_http::redirect::Redirect;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::started_session::StartedSession;

use crate::admits_sign_in::AdmitsSignIn;
use crate::sign_in_admission::SignInAdmission;
use crate::sign_in_completion::SignInCompletion;
use crate::sign_in_flow::SignInFlow;

pub struct SignInCallbackHandler<TAdmission> {
    admission: Arc<TAdmission>,
    flow: Arc<SignInFlow>,
    landing: String,
    sessions: Arc<IssuedSessions>,
}

impl<TAdmission: AdmitsSignIn> SignInCallbackHandler<TAdmission> {
    #[must_use]
    pub fn create(
        flow: Arc<SignInFlow>,
        admission: Arc<TAdmission>,
        sessions: Arc<IssuedSessions>,
        landing: String,
    ) -> Self {
        Self {
            admission,
            flow,
            landing,
            sessions,
        }
    }

    async fn answered(&self, request: &Request) -> anyhow::Result<ResponseContinuation> {
        let transaction_removal = self.flow.transaction_removal();
        let refused = match self.flow.complete::<TAdmission::IdClaims>(request).await {
            SignInCompletion::SignedIn(signed_in) => {
                match self.admission.admit(signed_in, &self.flow).await? {
                    SignInAdmission::Admitted(subject) => {
                        return self.started(subject, transaction_removal).await;
                    }
                    SignInAdmission::Refused => {
                        Response::text(403, "The signed-in identity is not admitted")
                    }
                }
            }
            SignInCompletion::Refused(refusal) => Response::text(403, refusal.to_string()),
            SignInCompletion::SigningKeysAwaited => Response::text(
                503,
                "The signing keys of the identity server are not available yet",
            ),
            SignInCompletion::Unavailable(unavailability) => {
                Response::text(503, unavailability.to_string())
            }
        };

        Ok(ResponseContinuation::from(
            refused.set_cookie(&transaction_removal),
        ))
    }

    async fn started(
        &self,
        subject: Uuid,
        transaction_removal: Cookie<'static>,
    ) -> anyhow::Result<ResponseContinuation> {
        let StartedSession {
            cookie_changes: CookieChanges { mut cookies },
            ..
        } = self.sessions.start(subject, Utc::now()).await?;

        cookies.push(transaction_removal);

        Ok(
            CookieChanges { cookies }.apply(ResponseContinuation::from(Redirect::see_other(
                self.landing.clone(),
            ))),
        )
    }
}

#[async_trait]
impl<TAdmission: AdmitsSignIn> HeadHandler for SignInCallbackHandler<TAdmission> {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.answered(request).await.map_err(HandlerError::consumer)
    }
}
