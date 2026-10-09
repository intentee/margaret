use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::consent_page_props::ConsentPageProps;
use margaret::framework::oidc_provider::consent_request::ConsentRequest;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::PreEscaped;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::routes::Routes;
use crate::routes::identity::consent_required::ConsentRequired;

#[renders_view(name = "consent_view")]
#[singleton]
pub struct ConsentView;

impl RendersView for ConsentView {
    type Props<'props> = ConsentPageProps<'props, Routes>;

    fn render(
        &self,
        ConsentPageProps {
            consent: ConsentRequest { id, .. },
            ..
        }: Self::Props<'_>,
    ) -> anyhow::Result<Markup> {
        Ok(PreEscaped(serde_json::to_string(&ConsentRequired {
            consent: *id,
        })?))
    }
}
