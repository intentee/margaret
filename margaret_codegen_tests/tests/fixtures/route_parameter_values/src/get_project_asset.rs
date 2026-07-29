use anyhow::Result;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::asset_bag::asset;

use super::project::Project;

#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_project_asset",
    path = "/projects/{project}/assets/{*asset_path}",
    server = "public"
)]
pub struct GetProjectAsset;

impl GetProjectAsset {
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "project")] Project { name }: Project,
        #[route_parameter(from = "asset_path")] asset_path: String,
    ) -> Result<Response> {
        Ok({
            let _ = asset!("resources/ts/app.ts");

            Response::text(200, format!("{name}:{asset_path}"))
        })
    }
}
