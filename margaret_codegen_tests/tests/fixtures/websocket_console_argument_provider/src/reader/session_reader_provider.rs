use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

#[singleton]
#[infers_authenticated_user(user_model = crate::reader::Reader)]
pub struct SessionReaderProvider {
    realm: Arc<crate::reader::reader_realm::ReaderRealm>,
}

impl SessionReaderProvider {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(realm: Arc<crate::reader::reader_realm::ReaderRealm>) -> anyhow::Result<Self> {
        Ok(Self { realm })
    }

    #[infer_from_request]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn infer_reader(
        &self,
        request: &Request,
        #[form_request(from = Cookie)] cookie: crate::reader::reader_cookie::ReaderCookie,
    ) -> anyhow::Result<AuthenticatedUserOutcome<crate::reader::Reader>> {
        let _ = request.inputs.server.path();

        let Some(reader) = cookie.reader else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };

        Ok(if self.realm.admits(&reader) {
            AuthenticatedUserOutcome::Authenticated(crate::reader::Reader { name: reader })
        } else {
            AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(Response::forbidden()))
        })
    }
}
