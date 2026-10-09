use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_validation::responded_to_form::responded_to_form;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::resolved_session::ResolvedSession;
use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;
use margaret_views::renders_view::RendersView;

use crate::authorization_endpoint::AuthorizationEndpoint;
use crate::authorization_outcome::AuthorizationOutcome;
use crate::authorization_request::AuthorizationRequest;
use crate::consent_page_props::ConsentPageProps;
use crate::end_user_authentication::EndUserAuthentication;
use crate::end_user_of::end_user_of;
use crate::frame_denied::frame_denied;

pub struct AuthorizationHandler<TConsentView, TRoutes> {
    authorization: Arc<AuthorizationEndpoint>,
    consent_view: Arc<TConsentView>,
    decision_url: String,
    routes: Arc<TRoutes>,
    sessions: Arc<IssuedSessions>,
}

impl<TConsentView, TRoutes> AuthorizationHandler<TConsentView, TRoutes>
where
    TConsentView:
        for<'page> RendersView<Props<'page> = ConsentPageProps<'page, TRoutes>> + Send + Sync,
    TRoutes: Send + Sync,
{
    #[must_use]
    pub fn create(
        authorization: Arc<AuthorizationEndpoint>,
        sessions: Arc<IssuedSessions>,
        consent_view: Arc<TConsentView>,
        decision_url: String,
        routes: Arc<TRoutes>,
    ) -> Self {
        Self {
            authorization,
            consent_view,
            decision_url,
            routes,
            sessions,
        }
    }

    async fn respond(
        &self,
        request: &Request,
        authorization_request: ValidationResult<AuthorizationRequest>,
    ) -> anyhow::Result<ResponseContinuation> {
        let ResolvedSession {
            cookie_changes,
            session,
        } = self.sessions.resolve(request).await?;
        let end_user = match session {
            Some(session) => EndUserAuthentication::Authenticated(end_user_of(session)),
            None => EndUserAuthentication::Anonymous,
        };
        let response = match self
            .authorization
            .authorize(authorization_request, &end_user)
            .await?
        {
            AuthorizationOutcome::AuthenticationRequired { return_to } => Response::text(
                401,
                format!("Sign in to the identity server, then continue at {return_to}"),
            ),
            AuthorizationOutcome::ConsentRequired(consent) => Response::html(
                200,
                self.consent_view
                    .render(ConsentPageProps {
                        consent: &consent,
                        decision_url: &self.decision_url,
                        routes: &self.routes,
                    })?
                    .into_string(),
            ),
            AuthorizationOutcome::Redirected(response)
            | AuthorizationOutcome::Rejected(response) => response,
        };

        Ok(cookie_changes.precede(ResponseContinuation::from(frame_denied(response))))
    }
}

#[async_trait]
impl<TConsentView, TRoutes> HeadHandler for AuthorizationHandler<TConsentView, TRoutes>
where
    TConsentView:
        for<'page> RendersView<Props<'page> = ConsentPageProps<'page, TRoutes>> + Send + Sync,
    TRoutes: Send + Sync,
{
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.respond(request, validate(&request.inputs.query))
            .await
            .map_err(HandlerError::consumer)
    }
}

#[async_trait]
impl<TConsentView, TRoutes> HandlesLimitedContent for AuthorizationHandler<TConsentView, TRoutes>
where
    TConsentView:
        for<'page> RendersView<Props<'page> = ConsentPageProps<'page, TRoutes>> + Send + Sync,
    TRoutes: Send + Sync,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
        limit: BodyLimit,
    ) -> Result<ResponseContinuation, HandlerError> {
        responded_to_form(request, body, limit, |authorization_request| {
            self.respond(request, authorization_request)
        })
        .await
    }
}
