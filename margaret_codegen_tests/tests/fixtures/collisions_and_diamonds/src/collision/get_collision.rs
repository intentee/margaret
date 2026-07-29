use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/collision", server = "public")]
pub struct GetCollision;

impl GetCollision {
    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn respond(
        &self,
        collision_session: &Request,
        #[authenticated_user] reader: crate::collision::reader::Reader,
    ) -> anyhow::Result<Response> {
        tokio::task::yield_now().await;

        Ok(Response::text(
            200,
            format!("{} {}", reader.name, collision_session.inputs.server.path()),
        ))
    }
}
