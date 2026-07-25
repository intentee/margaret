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

pub struct FarewellViewProps<'routes> {
    pub name: String,
    pub routes: &'routes Routes,
}

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
    type Props<'props> = FarewellViewProps<'props>;

    fn render(&self, FarewellViewProps { name, routes }: Self::Props<'_>) -> Markup {
        self.card_layout.render(CardLayoutProps {
            body: html! { "goodbye, " (name) },
            home_url: routes.public.get_greeting.url(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret::framework::views::renders_view::RendersView;

    use super::FarewellView;
    use super::FarewellViewProps;
    use crate::margaret::routes::Routes;
    use crate::views::card_layout::CardLayout;

    #[test]
    fn renders_the_farewell_inside_the_card_layout() {
        let view = FarewellView {
            card_layout: Arc::new(CardLayout),
        };
        let routes = Routes::from_origins(Arc::from("http://internal"), Arc::from("http://public"));

        let markup = view.render(FarewellViewProps {
            name: "Ada".to_string(),
            routes: &routes,
        });

        assert_eq!(
            markup.into_string(),
            "<main>goodbye, Ada</main><nav><a href=\"http://public/greeting\">home</a></nav>"
        );
    }
}
