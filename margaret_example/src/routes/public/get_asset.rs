use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::asset_bag::AssetServer;

#[singleton]
#[responds_to_http(method = "get", path = "/assets/{*asset_path}", server = "public")]
pub struct GetAsset {
    assets: AssetServer,
}

impl GetAsset {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self {
            assets: AssetServer::new(),
        }
    }

    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "asset_path")] asset_path: String,
    ) -> Response {
        self.assets.respond(&asset_path)
    }
}

#[cfg(test)]
mod tests {
    use super::GetAsset;

    #[tokio::test]
    async fn serves_an_embedded_hashed_asset() {
        assert_eq!(
            GetAsset::create()
                .respond("logo_I9J0K1L2.png".to_string())
                .await
                .status(),
            200
        );
    }

    #[tokio::test]
    async fn responds_with_not_found_for_a_missing_asset() {
        assert_eq!(
            GetAsset::create()
                .respond("missing.png".to_string())
                .await
                .status(),
            404
        );
    }
}
