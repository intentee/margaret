use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::identity::authenticated_user_inference::AuthenticatedUserInference;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

pub struct Reader {
    pub name: String,
}

#[singleton]
#[infers_authenticated_user(
    login_route = crate::collision::SignIn,
    user_model = crate::collision::Reader
)]
pub struct Session;

impl Session {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    #[infer_from_request]
    pub async fn infer_reader(&self) -> anyhow::Result<AuthenticatedUserInference<Reader>> {
        Ok(AuthenticatedUserInference::Authenticated(Reader {
            name: "milo".to_string(),
        }))
    }
}

#[singleton]
#[responds_to_http(method = "get", name = "sign_in", path = "/sign-in", server = "public")]
pub struct SignIn;

impl SignIn {
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, "sign in"))
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
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            format!("{} {}", reader.name, collision_session.inputs.server.path()),
        ))
    }
}
