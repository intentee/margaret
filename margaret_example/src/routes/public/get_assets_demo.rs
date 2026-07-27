use std::sync::Arc;

use margaret::framework::asset_bag::asset_bag::AssetBag;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::views::renders_view::RendersView;

use crate::views::asset_page::AssetPage;
use crate::views::asset_page::AssetPageProps;

#[singleton]
#[responds_to_http(access = margaret::framework::http::public_access::PublicAccess, method = "get", path = "/assets-demo", server = "public")]
pub struct GetAssetsDemo {
    asset_page: Arc<AssetPage>,
}

impl GetAssetsDemo {
    #[constructor]
    pub fn create(asset_page: Arc<AssetPage>) -> anyhow::Result<Self> {
        Ok(Self { asset_page })
    }

    #[process]
    pub async fn respond(&self, asset_bag: AssetBag) -> anyhow::Result<Response> {
        Ok(Response::html(
            200,
            self.asset_page.render(AssetPageProps { asset_bag })?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret::framework::asset_bag::asset_bag::AssetBag;

    use super::GetAssetsDemo;
    use crate::views::asset_page::AssetPage;
    use crate::views::asset_showcase::AssetShowcase;

    #[tokio::test]
    async fn responds_with_the_rendered_asset_page() {
        let page =
            AssetPage::create(Arc::new(AssetShowcase)).expect("the asset page is constructed");
        let responder =
            GetAssetsDemo::create(Arc::new(page)).expect("the responder is constructed");

        assert_eq!(
            responder
                .respond(AssetBag::new())
                .await
                .expect("the responder succeeds")
                .status(),
            200
        );
    }
}
