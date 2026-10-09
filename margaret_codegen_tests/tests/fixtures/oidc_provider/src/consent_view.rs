use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::consent_page_props::ConsentPageProps;
use margaret::framework::oidc_provider::consent_request::ConsentRequest;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::routes::Routes;

#[renders_view(name = "consent_view")]
#[singleton]
pub struct ConsentView;

impl RendersView for ConsentView {
    type Props<'props> = ConsentPageProps<'props, Routes>;

    fn render(
        &self,
        ConsentPageProps {
            consent: ConsentRequest { client_id, id, .. },
            decision_url,
            routes,
        }: Self::Props<'_>,
    ) -> anyhow::Result<Markup> {
        Ok(html! {
            form method="post" action=(decision_url) {
                input type="hidden" name="id" value=(id);
                p { (client_id) }
            }
            a href=(routes.public.get_discovery.url()) { "discovery" }
        })
    }
}
