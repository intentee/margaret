use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_identity::responds_to_inference_failure::RespondsToInferenceFailure;
use margaret_macros::constructor;
use margaret_macros::infer_from_request;
use margaret_macros::infers_authenticated_user;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

pub struct Reader {
    pub name: String,
}

pub struct ReaderUnavailable;

impl RespondsToInferenceFailure for ReaderUnavailable {
    fn into_response_continuation(self) -> ResponseContinuation {
        ResponseContinuation::from(Response::forbidden())
    }
}

#[singleton]
#[infers_authenticated_user(user_model = crate::collision::Reader)]
pub struct Session;

impl Session {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    #[infer_from_request]
    pub async fn infer_reader(
        &self,
    ) -> Result<AuthenticatedUserOutcome<Reader>, ReaderUnavailable> {
        Ok(AuthenticatedUserOutcome::Authenticated(Reader {
            name: "milo".to_string(),
        }))
    }
}

#[singleton]
#[responds_to_http(method = "get", path = "/collision", server = "public")]
pub struct GetCollision;

impl GetCollision {
    #[process]
    pub async fn respond(
        &self,
        collision_session: &Request,
        #[authenticated_user] reader: Reader,
    ) -> Response {
        Response::text(
            200,
            format!("{} {}", reader.name, collision_session.inputs.server.path()),
        )
    }
}
