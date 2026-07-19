use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::renders_view;
use margaret_macros::singleton;
use margaret_views::maud::Markup;
use margaret_views::maud::html;
use margaret_views::renders_view::RendersView;

use crate::views::card_layout::CardLayout;
use crate::views::card_layout_props::CardLayoutProps;
use crate::views::greeting_view_props::GreetingViewProps;

#[renders_view(name = "greeting_view")]
#[singleton]
pub struct GreetingView {
    card_layout: Arc<CardLayout>,
}

impl GreetingView {
    #[constructor]
    #[must_use]
    pub fn create(card_layout: Arc<CardLayout>) -> Self {
        Self { card_layout }
    }
}

impl RendersView for GreetingView {
    type Props = GreetingViewProps;

    fn render(&self, GreetingViewProps { greeting, home_url }: GreetingViewProps) -> Markup {
        self.card_layout.render(CardLayoutProps {
            body: html! { (greeting) },
            home_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_views::renders_view::RendersView;

    use super::GreetingView;
    use crate::views::card_layout::CardLayout;
    use crate::views::greeting_view_props::GreetingViewProps;

    #[test]
    fn renders_the_greeting_inside_the_card_layout() {
        let view = GreetingView {
            card_layout: Arc::new(CardLayout),
        };

        let markup = view.render(GreetingViewProps {
            greeting: "Hello, World".to_string(),
            home_url: "/greeting".to_string(),
        });

        assert_eq!(
            markup.into_string(),
            "<main>Hello, World</main><nav><a href=\"/greeting\">home</a></nav>"
        );
    }
}
