use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::renders_view;
use margaret_macros::singleton;
use margaret_views::maud::Markup;
use margaret_views::maud::html;
use margaret_views::renders_view::RendersView;

use crate::views::card_layout::CardLayout;
use crate::views::card_layout_props::CardLayoutProps;
use crate::views::farewell_view_props::FarewellViewProps;

#[renders_view(name = "farewell_view")]
#[singleton]
pub struct FarewellView {
    card_layout: Arc<CardLayout>,
}

impl FarewellView {
    #[constructor]
    #[must_use]
    pub fn create(card_layout: Arc<CardLayout>) -> Self {
        Self { card_layout }
    }
}

impl RendersView for FarewellView {
    type Props = FarewellViewProps;

    fn render(&self, FarewellViewProps { home_url, name }: FarewellViewProps) -> Markup {
        self.card_layout.render(CardLayoutProps {
            body: html! { "goodbye, " (name) },
            home_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_views::renders_view::RendersView;

    use super::FarewellView;
    use crate::views::card_layout::CardLayout;
    use crate::views::farewell_view_props::FarewellViewProps;

    #[test]
    fn renders_the_farewell_inside_the_card_layout() {
        let view = FarewellView {
            card_layout: Arc::new(CardLayout),
        };

        let markup = view.render(FarewellViewProps {
            home_url: "/greeting".to_string(),
            name: "Ada".to_string(),
        });

        assert_eq!(
            markup.into_string(),
            "<main>goodbye, Ada</main><nav><a href=\"/greeting\">home</a></nav>"
        );
    }
}
