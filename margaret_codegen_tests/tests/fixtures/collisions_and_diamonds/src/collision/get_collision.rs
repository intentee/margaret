use tokio::task;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::collision::reader::Reader;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/collision", server = "public")]
pub struct GetCollision;

impl GetCollision {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        collision_session: &Request,
        #[authenticated_user] reader: Reader,
    ) -> anyhow::Result<Response> {
        task::yield_now().await;

        Ok(Response::text(
            200,
            format!("{} {}", reader.name, collision_session.inputs.server.path()),
        ))
    }
}
