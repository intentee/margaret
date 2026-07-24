use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_identity::responds_to_inference_failure::RespondsToInferenceFailure;
use margaret_macros::constructor;
use margaret_macros::infer_from_request;
use margaret_macros::infers_authenticated_user;
use margaret_macros::singleton;
use serde::Deserialize;
use thiserror::Error;
use validator::Validate;

#[derive(Clone)]
pub struct Reader {
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct ReaderCookie {
    pub reader: Option<String>,
}

#[derive(Debug, Error)]
pub enum ReaderError {
    #[error("the reader '{reader}' is not admitted to the realm '{realm}'")]
    OutsideRealm { realm: String, reader: String },
}

impl RespondsToInferenceFailure for ReaderError {
    fn into_response_continuation(self) -> ResponseContinuation {
        ResponseContinuation::from(Response::forbidden())
    }
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

    #[must_use]
    pub fn name(&self) -> &str {
        &self.realm
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
    ) -> Result<AuthenticatedUserOutcome<Reader>, ReaderError> {
        let _ = request.inputs.server.path();

        let Some(reader) = cookie.reader else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };

        if self.realm.admits(&reader) {
            Ok(AuthenticatedUserOutcome::Authenticated(Reader {
                name: reader,
            }))
        } else {
            Err(ReaderError::OutsideRealm {
                realm: self.realm.name().to_string(),
                reader,
            })
        }
    }
}
