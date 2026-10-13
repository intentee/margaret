use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::token_introspection::introspected_token::IntrospectedToken;

use crate::artifact_claims::ArtifactClaims;
use crate::artifact_uploader::ArtifactUploader;

#[singleton]
#[infers_authenticated_user(user_model = ArtifactUploader)]
pub struct ArtifactUploaderProvider;

impl ArtifactUploaderProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer(
        &self,
        #[bearer_token(client = partner_client)] token: Option<IntrospectedToken<ArtifactClaims>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<ArtifactUploader>> {
        Ok(match token {
            Some(introspected) => AuthenticatedUserOutcome::Authenticated(ArtifactUploader {
                repository: introspected.claims.repository,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
