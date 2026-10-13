use anyhow::Result;

use margaret::framework::asset_bag::asset_bag::AssetBag;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use super::project::Project;
use crate::margaret::asset_bag::asset;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_project_asset",
    path = "/projects/{project}/assets/{*asset_path}",
    server = "public"
)]
pub struct GetProjectAsset;

impl GetProjectAsset {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "project")] Project { name }: Project,
        #[route_parameter(from = "asset_path")] asset_path: String,
    ) -> Result<Response> {
        let logo = AssetBag::new().image(asset!("resources/media/logo.png"));

        Ok(Response::text(200, [name, asset_path, logo].join(":")))
    }
}
