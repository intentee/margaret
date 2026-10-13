use maud::Markup;

use margaret_oidc_provider::consent_page_props::ConsentPageProps;
use margaret_views::renders_view::RendersView;

use crate::fixture_routes::FixtureRoutes;

pub struct FailingConsentView;

impl RendersView for FailingConsentView {
    type Props<'props> = ConsentPageProps<'props, FixtureRoutes>;

    fn render(&self, _props: Self::Props<'_>) -> anyhow::Result<Markup> {
        Err(anyhow::anyhow!("the consent page cannot be rendered"))
    }
}
