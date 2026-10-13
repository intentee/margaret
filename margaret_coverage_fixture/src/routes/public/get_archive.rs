use margaret::framework::http::response::Response;
use margaret::framework::macros::middleware;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

#[singleton]
#[middleware(archive_visit)]
#[responds_to_http(method = RouteMethod::Get, path = "/archive", server = "public")]
pub struct GetArchive;

impl GetArchive {
    /// # Errors
    ///
    /// Never returns an error; the archive always opens to a checked visit.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, "the archive is open"))
    }
}
