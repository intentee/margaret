use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_views::renders_view::RendersView;

use crate::greeter::Greeter;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::views::greeting_view_props::GreetingViewProps;

#[singleton]
#[responds_to_http(method = "get", path = "/greeting-card", server = "public")]
pub struct GetGreetingCard {
    greeter: Arc<dyn Greeter>,
}

impl GetGreetingCard {
    #[constructor]
    pub fn create(greeter: Arc<dyn Greeter>) -> Self {
        Self { greeter }
    }

    #[process]
    pub async fn respond(&self, routes: &Routes, views: &Views) -> Response {
        Response::html(
            200,
            views.greeting_view.render(GreetingViewProps {
                greeting: self.greeter.greet(),
                home_url: routes.public.get_greeting.url(),
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::GetGreetingCard;
    use crate::greeter::Greeter;
    use crate::margaret::routes::Routes;
    use crate::margaret::views::Views;
    use crate::views::card_layout::CardLayout;
    use crate::views::farewell_view::FarewellView;
    use crate::views::greeting_view::GreetingView;

    struct StaticGreeter;

    impl Greeter for StaticGreeter {
        fn greet(&self) -> String {
            "Hello, World".to_string()
        }
    }

    #[tokio::test]
    async fn responds_with_the_rendered_greeting_card() {
        let card_layout = Arc::new(CardLayout);
        let views = Views {
            card_layout: card_layout.clone(),
            farewell_view: Arc::new(FarewellView::create(card_layout.clone())),
            greeting_view: Arc::new(GreetingView::create(card_layout)),
        };
        let routes = Routes::from_origins(Arc::from("http://internal"), Arc::from("http://public"));
        let responder = GetGreetingCard::create(Arc::new(StaticGreeter));

        assert_eq!(responder.respond(&routes, &views).await.status(), 200);
    }
}
