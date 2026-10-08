use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::asset_bag::asset_responder::AssetResponder;
use crate::routes::public::asset_path::AssetPath;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_asset",
    path = "/assets/{*asset_path}",
    server = "public"
)]
pub struct GetAsset {
    asset_responder: Arc<AssetResponder>,
}

impl GetAsset {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(asset_responder: Arc<AssetResponder>) -> anyhow::Result<Self> {
        Ok(Self { asset_responder })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "asset_path")] AssetPath(asset_path): AssetPath,
    ) -> anyhow::Result<Response> {
        Ok(self.asset_responder.respond(&asset_path))
    }
}
