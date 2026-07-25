use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::routes::Routes;
use crate::views::card_layout::CardLayout;
use crate::views::card_layout::CardLayoutProps;

pub struct GreetingViewProps<'routes> {
    pub greeting: String,
    pub routes: &'routes Routes,
}

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
    type Props<'props> = GreetingViewProps<'props>;

    fn render(&self, GreetingViewProps { greeting, routes }: Self::Props<'_>) -> Markup {
        self.card_layout.render(CardLayoutProps {
            body: html! { (greeting) },
            home_url: routes.public.get_greeting.url(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret::framework::views::renders_view::RendersView;

    use super::GreetingView;
    use super::GreetingViewProps;
    use crate::margaret::routes::Routes;
    use crate::views::card_layout::CardLayout;

    #[test]
    fn renders_the_greeting_inside_the_card_layout() {
        let view = GreetingView {
            card_layout: Arc::new(CardLayout),
        };
        let routes = Routes::from_origins(Arc::from("http://internal"), Arc::from("http://public"));

        let markup = view.render(GreetingViewProps {
            greeting: "Hello, World".to_string(),
            routes: &routes,
        });

        assert_eq!(
            markup.into_string(),
            "<main>Hello, World</main><nav><a href=\"http://public/greeting\">home</a></nav>"
        );
    }
}
