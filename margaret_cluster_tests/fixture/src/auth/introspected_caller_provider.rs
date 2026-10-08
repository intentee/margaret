use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::token_introspection::introspected_token::IntrospectedToken;

use crate::auth::introspected_claims::IntrospectedClaims;
use crate::models::introspected_caller::IntrospectedCaller;

#[singleton]
#[infers_authenticated_user(user_model = IntrospectedCaller)]
pub struct IntrospectedCallerProvider;

impl IntrospectedCallerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_introspected_caller(
        &self,
        #[bearer_token(client = cluster)] token: Option<IntrospectedToken<IntrospectedClaims>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<IntrospectedCaller>> {
        Ok(match token {
            Some(IntrospectedToken {
                subject: Some(subject),
                ..
            }) => AuthenticatedUserOutcome::Authenticated(IntrospectedCaller { subject }),
            Some(IntrospectedToken { subject: None, .. }) => AuthenticatedUserOutcome::Interrupted(
                ResponseContinuation::from(Response::forbidden()),
            ),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
