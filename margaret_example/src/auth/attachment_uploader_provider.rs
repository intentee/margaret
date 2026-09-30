use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::token_introspection::introspected_token::IntrospectedToken;

use crate::auth::attachment_claims::AttachmentClaims;
use crate::models::attachment_uploader::AttachmentUploader;

#[singleton]
#[infers_authenticated_user(user_model = AttachmentUploader)]
pub struct AttachmentUploaderProvider;

impl AttachmentUploaderProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_attachment_uploader(
        &self,
        #[bearer_token(client = blog)] token: Option<IntrospectedToken<AttachmentClaims>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<AttachmentUploader>> {
        Ok(match token {
            Some(IntrospectedToken {
                subject: Some(subject),
                ..
            }) => AuthenticatedUserOutcome::Authenticated(AttachmentUploader { subject }),
            Some(IntrospectedToken { subject: None, .. }) => AuthenticatedUserOutcome::Interrupted(
                ResponseContinuation::from(Response::forbidden()),
            ),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
