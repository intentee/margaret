use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

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
        #[bearer_token(resource = attachments)] token: Option<
            VerifiedJwt<AttachmentClaims, AccessTokenProfile>,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<AttachmentUploader>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(AttachmentUploader {
                subject: verified.claims.sub,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
