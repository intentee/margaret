use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

pub use super::home_view_props::HomeViewProps;
use super::secrets::Secrets;

#[renders_view(name = "home_view")]
#[singleton]
pub struct HomeView {
    secrets: Arc<Secrets>,
}

impl HomeView {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> failures::Result<Self> {
        Ok(Self { secrets })
    }
}

impl RendersView for HomeView {
    type Props<'props> = HomeViewProps;

    fn render(
        &self,
        HomeViewProps { heading }: Self::Props<'_>,
    ) -> std::result::Result<Markup, failures::Error> {
        Ok({
            let _ = self.secrets.token();

            html! { h1 { (heading) } }
        })
    }
}
