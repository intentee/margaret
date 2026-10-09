use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_validation::responded_to_form::responded_to_form;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_validation::validation_result::ValidationResult;

use crate::consent_endpoint::ConsentEndpoint;
use crate::consent_outcome::ConsentOutcome;
use crate::consent_submission::ConsentSubmission;
use crate::end_user_of::end_user_of;

pub struct ConsentHandler {
    consent: Arc<ConsentEndpoint>,
    sessions: Arc<IssuedSessions>,
}

impl ConsentHandler {
    #[must_use]
    pub fn create(consent: Arc<ConsentEndpoint>, sessions: Arc<IssuedSessions>) -> Self {
        Self { consent, sessions }
    }

    async fn respond(
        &self,
        request: &Request,
        submission: ValidationResult<ConsentSubmission>,
    ) -> anyhow::Result<ResponseContinuation> {
        let ResolvedSession {
            cookie_changes,
            session,
        } = self.sessions.resolve(request).await?;
        let Some(session) = session else {
            return Ok(cookie_changes.apply(ResponseContinuation::from(Response::unauthorized())));
        };
        let response = match submission {
            ValidationResult::Valid(ConsentSubmission { decision, id }) => {
                match self
                    .consent
                    .decide(id, &end_user_of(session), decision.decision())
                    .await?
                {
                    ConsentOutcome::Redirected(response) => response,
                    ConsentOutcome::Unknown => Response::text(
                        404,
                        "The authorization request is unknown or was already decided",
                    ),
                }
            }
            ValidationResult::Invalid(_) | ValidationResult::Malformed(_) => {
                Response::text(422, "The consent decision is malformed")
            }
        };

        Ok(cookie_changes.apply(ResponseContinuation::from(response)))
    }
}

#[async_trait]
impl HandlesLimitedContent for ConsentHandler {
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
