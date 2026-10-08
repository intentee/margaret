use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::notes_caller_claims::NotesCallerClaims;
use crate::models::notes_caller::NotesCaller;

#[singleton]
#[infers_authenticated_user(user_model = NotesCaller)]
pub struct NotesCallerProvider;

impl NotesCallerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_notes_caller(
        &self,
        #[bearer_token(resource = notes)] token: Option<
            VerifiedJwt<NotesCallerClaims, AccessTokenProfile>,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<NotesCaller>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(NotesCaller {
                subject: verified.claims.sub,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
