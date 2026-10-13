use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::mesh_caller::MeshCaller;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/caller", server = "mesh")]
pub struct GetMeshCaller;

impl GetMeshCaller {
    /// # Errors
    ///
    /// Never returns an error; every admitted mesh caller is named.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] MeshCaller { role, spiffe_id }: MeshCaller,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("{role} {spiffe_id}")))
    }
}
