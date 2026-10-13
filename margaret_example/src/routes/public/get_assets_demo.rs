use std::sync::Arc;

use margaret::framework::asset_bag::asset_bag::AssetBag;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::views::renders_view::RendersView;

use crate::views::asset_page::AssetPage;
use crate::views::asset_page::AssetPageProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/assets-demo", server = "public")]
pub struct GetAssetsDemo {
    asset_page: Arc<AssetPage>,
}

impl GetAssetsDemo {
    #[must_use]
    #[constructor]
    pub fn create(asset_page: Arc<AssetPage>) -> Self {
        Self { asset_page }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, asset_bag: AssetBag) -> anyhow::Result<Response> {
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
    async fn responds_ok_with_the_rendered_asset_page_so_the_demo_route_serves_a_page() {
        let page = AssetPage::create(Arc::new(AssetShowcase));
        let responder = GetAssetsDemo::create(Arc::new(page));

        assert_eq!(
            responder
                .respond(AssetBag::new())
                .expect("the responder succeeds")
                .status(),
            200
        );
    }
}
