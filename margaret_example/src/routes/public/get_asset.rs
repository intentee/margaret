use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::asset_bag::asset_responder::AssetResponder;

#[singleton]
#[responds_to_http(method = "get", path = "/assets/{*asset_path}", server = "public")]
pub struct GetAsset {
    asset_responder: Arc<AssetResponder>,
}

impl GetAsset {
    #[constructor]
    #[must_use]
    pub fn create(asset_responder: Arc<AssetResponder>) -> Self {
        Self { asset_responder }
    }

    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "asset_path")] asset_path: String,
    ) -> Response {
        self.asset_responder.respond(&asset_path)
    }
}
