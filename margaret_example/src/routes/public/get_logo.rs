use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/logo.png", server = "public")]
pub struct GetLogo;

impl GetLogo {
    #[process]
    pub async fn respond(&self) -> Response {
        Response::bytes(
            200,
            "image/png",
            include_bytes!("../../../assets/logo.png").as_slice(),
        )
    }
}
