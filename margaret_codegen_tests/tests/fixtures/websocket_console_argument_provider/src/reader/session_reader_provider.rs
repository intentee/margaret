use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::reader::Reader;
use crate::reader::reader_cookie::ReaderCookie;
use crate::reader::reader_realm::ReaderRealm;

#[singleton]
#[infers_authenticated_user(user_model = Reader)]
pub struct SessionReaderProvider {
    realm: Arc<ReaderRealm>,
}

impl SessionReaderProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(realm: Arc<ReaderRealm>) -> anyhow::Result<Self> {
        Ok(Self { realm })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_reader(
        &self,
        request: &Request,
        #[form_request(from = Cookie)] cookie: ReaderCookie,
    ) -> anyhow::Result<AuthenticatedUserOutcome<Reader>> {
        let _ = request.inputs.server.path();

        let Some(reader) = cookie.reader else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };

        Ok(if self.realm.admits(&reader) {
            AuthenticatedUserOutcome::Authenticated(Reader { name: reader })
        } else {
            AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(Response::forbidden()))
        })
    }
}
