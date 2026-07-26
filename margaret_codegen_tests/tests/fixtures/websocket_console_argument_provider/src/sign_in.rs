use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "get", name = "sign_in", path = "/sign-in", server = "public")]
pub struct SignIn;

impl SignIn {
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, "sign in"))
    }
}
