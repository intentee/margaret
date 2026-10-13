use maud::Markup;
use maud::html;

use margaret_oidc_provider::consent_page_props::ConsentPageProps;
use margaret_oidc_provider::consent_request::ConsentRequest;
use margaret_views::renders_view::RendersView;

use crate::fixture_routes::FixtureRoutes;

pub struct FixtureConsentView;

impl RendersView for FixtureConsentView {
    type Props<'props> = ConsentPageProps<'props, FixtureRoutes>;

    fn render(
        &self,
        ConsentPageProps {
            consent: ConsentRequest { client_id, id, .. },
            decision_url,
            routes: FixtureRoutes { home_url },
        }: Self::Props<'_>,
    ) -> anyhow::Result<Markup> {
        Ok(html! {
            form method="post" action=(decision_url) {
                input type="hidden" name="id" value=(id);
                p { (client_id) }
            }
            a href=(home_url) { "home" }
        })
    }
}
