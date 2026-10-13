use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::clients::mesh_client::MeshClient;
use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/relay", server = "public")]
pub struct GetRelay {
    mesh_client: Arc<MeshClient>,
}

impl GetRelay {
    #[must_use]
    #[constructor]
    pub fn create(mesh_client: Arc<MeshClient>) -> Self {
        Self { mesh_client }
    }

    /// # Errors
    ///
    /// Returns an error when the mesh cannot be reached.
    #[process]
    pub async fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {
        self.mesh_client
            .fetch(routes.mesh.get_peer.url())
            .await
            .map(|peer| Response::text(200, peer))
            .map_err(anyhow::Error::from)
    }
}
