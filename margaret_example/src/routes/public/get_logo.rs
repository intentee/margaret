use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/logo.png", server = "public")]
pub struct GetLogo;

impl GetLogo {
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok({
            Response::bytes(
                200,
                "image/png",
                include_bytes!("../../../../assets/logo.png").as_slice(),
            )
        })
    }
}
