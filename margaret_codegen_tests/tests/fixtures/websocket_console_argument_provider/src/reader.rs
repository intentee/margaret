use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_macros::constructor;
use margaret_macros::infer_from_request;
use margaret_macros::infers_authenticated_user;
use margaret_macros::singleton;
use serde::Deserialize;
use validator::Validate;

#[derive(Clone)]
pub struct Reader {
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct ReaderCookie {
    pub reader: Option<String>,
}

#[singleton]
pub struct ReaderRealm {
    realm: String,
}

impl ReaderRealm {
    #[constructor]
    #[must_use]
    pub fn create(#[console_argument(from = "realm")] realm: String) -> Self {
        Self { realm }
    }

    #[must_use]
    pub fn admits(&self, reader: &str) -> bool {
        reader.starts_with(&self.realm)
    }
}

#[singleton]
#[infers_authenticated_user(user_model = crate::reader::Reader)]
pub struct SessionReaderProvider {
    realm: Arc<ReaderRealm>,
}

impl SessionReaderProvider {
    #[constructor]
    #[must_use]
    pub fn create(realm: Arc<ReaderRealm>) -> Self {
        Self { realm }
    }

    #[infer_from_request]
    pub async fn infer_reader(
        &self,
        request: &Request,
        #[form_request(from = Cookie)] cookie: ReaderCookie,
    ) -> AuthenticatedUserOutcome<Reader> {
        let _ = request.inputs.server.path();

        let Some(reader) = cookie.reader else {
            return AuthenticatedUserOutcome::Anonymous;
        };

        if self.realm.admits(&reader) {
            AuthenticatedUserOutcome::Authenticated(Reader { name: reader })
        } else {
            AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(Response::forbidden()))
        }
    }
}
