use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use super::margaret::asset_bag::asset;
use super::secrets::Secrets;

pub struct HomeViewProps {
    pub heading: String,
}

#[renders_view(name = "home_view")]
#[singleton]
pub struct HomeView {
    secrets: Arc<Secrets>,
}

impl HomeView {
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> anyhow::Result<Self> {
        Ok(Self { secrets })
    }
}

impl RendersView for HomeView {
    type Props<'props> = HomeViewProps;

    fn render(&self, HomeViewProps { heading }: Self::Props<'_>) -> anyhow::Result<Markup> {
        Ok({
            let _ = self.secrets.token();
            let _ = asset!("resources/ts/app.ts");

            html! { h1 { (heading) } }
        })
    }
}
