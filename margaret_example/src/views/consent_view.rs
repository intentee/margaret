use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::consent_page_props::ConsentPageProps;
use margaret::framework::oidc_provider::consent_request::ConsentRequest;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::routes::Routes;
use crate::views::card_layout::CardLayout;
use crate::views::card_layout::CardLayoutProps;

#[renders_view(name = "consent_view")]
#[singleton]
pub struct ConsentView {
    card_layout: Arc<CardLayout>,
}

impl ConsentView {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(card_layout: Arc<CardLayout>) -> anyhow::Result<Self> {
        Ok(Self { card_layout })
    }
}

impl RendersView for ConsentView {
    type Props<'props> = ConsentPageProps<'props, Routes>;

    fn render(
        &self,
        ConsentPageProps {
            consent:
                ConsentRequest {
                    client_id,
                    id,
                    scopes,
                },
            decision_url,
            routes,
        }: Self::Props<'_>,
    ) -> anyhow::Result<Markup> {
        self.card_layout.render(CardLayoutProps {
            body: html! {
                p { "The client " (client_id) " asks to act on your behalf." }
                ul {
                    @for scope in scopes {
                        li { (scope.as_str()) }
                    }
                }
                form method="post" action=(decision_url) {
                    input type="hidden" name="id" value=(id);
                    button type="submit" name="decision" value="approve" { "Approve" }
                    button type="submit" name="decision" value="deny" { "Deny" }
                }
            },
            home_url: routes.public.get_greeting.url(),
        })
    }
}
