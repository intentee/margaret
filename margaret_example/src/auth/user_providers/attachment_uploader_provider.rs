use margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::attachment_uploader::AttachmentUploader;
use crate::auth::claims::attachment_claims::AttachmentClaims;

#[singleton]
#[infers_authenticated_user(user_model = AttachmentUploader)]
pub struct AttachmentUploaderProvider;

impl AttachmentUploaderProvider {
    #[must_use]
    #[infer_from_request]
    pub fn infer_attachment_uploader(
        &self,
        #[bearer_token(resource = attachments)] token: Option<
            VerifiedJwt<AttachmentClaims, AccessTokenProfile>,
        >,
    ) -> AuthenticatedUserPresence<AttachmentUploader> {
        match token {
            Some(verified) => AuthenticatedUserPresence::Authenticated(AttachmentUploader {
                subject: verified.claims.sub,
            }),
            None => AuthenticatedUserPresence::Anonymous,
        }
    }
}
