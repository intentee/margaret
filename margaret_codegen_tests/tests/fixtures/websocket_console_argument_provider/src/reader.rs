use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
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
    pub fn create(#[console_argument(from = "realm")] realm: String) -> anyhow::Result<Self> {
        Ok(Self { realm })
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
    pub fn create(realm: Arc<ReaderRealm>) -> anyhow::Result<Self> {
        Ok(Self { realm })
    }

    #[infer_from_request]
    pub async fn infer_reader(
        &self,
        request: &Request,
        #[form_request(from = Cookie)] cookie: ReaderCookie,
    ) -> anyhow::Result<AuthenticatedUserOutcome<Reader>> {
        Ok({
            let _ = request.inputs.server.path();

            let Some(reader) = cookie.reader else {
                return Ok(AuthenticatedUserOutcome::Anonymous);
            };

            if self.realm.admits(&reader) {
                AuthenticatedUserOutcome::Authenticated(Reader { name: reader })
            } else {
                AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(
                    Response::forbidden(),
                ))
            }
        })
    }
}
