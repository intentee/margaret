use std::sync::Arc;

use margaret_asset_bag::asset_bag::AssetBag;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_views::renders_view::RendersView;

use crate::views::asset_page::AssetPage;
use crate::views::asset_page_props::AssetPageProps;

#[singleton]
#[responds_to_http(method = "get", path = "/assets-demo", server = "public")]
pub struct GetAssetsDemo {
    asset_page: Arc<AssetPage>,
}

impl GetAssetsDemo {
    #[constructor]
    #[must_use]
    pub fn create(asset_page: Arc<AssetPage>) -> Self {
        Self { asset_page }
    }

    #[process]
    pub async fn respond(&self, asset_bag: AssetBag) -> Response {
        Response::html(200, self.asset_page.render(AssetPageProps { asset_bag }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_asset_bag::asset_bag::AssetBag;

    use super::GetAssetsDemo;
    use crate::views::asset_page::AssetPage;
    use crate::views::asset_showcase::AssetShowcase;

    #[tokio::test]
    async fn responds_with_the_rendered_asset_page() {
        let responder = GetAssetsDemo::create(Arc::new(AssetPage::create(Arc::new(AssetShowcase))));

        assert_eq!(responder.respond(AssetBag::new()).await.status(), 200);
    }
}
