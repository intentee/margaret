use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::no_store::no_store;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::require_peer_spiffe_id::require_peer_spiffe_id;
use margaret_http::requirement::Requirement;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_validation::responded_to_form::responded_to_form;
use margaret_validation::validation_result::ValidationResult;

use crate::issued_sessions::IssuedSessions;
use crate::session_refresh::SessionRefresh;
use crate::session_refresh_answer::SessionRefreshAnswer;
use crate::session_refresh_request::SessionRefreshRequest;
use crate::sessions_error::SessionsError;

pub struct SessionRefreshEndpoint {
    sessions: Arc<IssuedSessions>,
}

impl SessionRefreshEndpoint {
    #[must_use]
    pub fn create(sessions: Arc<IssuedSessions>) -> Self {
        Self { sessions }
    }

    async fn respond(
        &self,
        request: ValidationResult<SessionRefreshRequest>,
    ) -> Result<Response, SessionsError> {
        let ValidationResult::Valid(SessionRefreshRequest { secret }) = request else {
            return Ok(Response::text(
                400,
                "The session refresh request is malformed",
            ));
        };

        Ok(match self.sessions.refresh(&secret).await? {
            SessionRefresh::Refreshed(access_token) => no_store(Response::json(
                200,
                &SessionRefreshAnswer {
                    access_token: access_token.signed_claims,
                },
            )),
            SessionRefresh::Refused => no_store(Response::unauthorized()),
        })
    }
}

#[async_trait]
impl HandlesLimitedContent for SessionRefreshEndpoint {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
        limit: BodyLimit,
    ) -> Result<ResponseContinuation, HandlerError> {
        if let Requirement::Unmet(refusal) = require_peer_spiffe_id(request) {
            return Ok(refusal);
        }

        responded_to_form(request, body, limit, |refresh| self.respond(refresh)).await
    }
}
