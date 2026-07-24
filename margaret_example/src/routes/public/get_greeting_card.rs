use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_views::renders_view::RendersView;

use crate::english_greeter::EnglishGreeter;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::views::greeting_view::GreetingViewProps;

#[singleton]
#[responds_to_http(method = "get", path = "/greeting-card", server = "public")]
pub struct GetGreetingCard {
    greeter: Arc<EnglishGreeter>,
}

impl GetGreetingCard {
    #[constructor]
    #[must_use]
    pub fn create(greeter: Arc<EnglishGreeter>) -> Self {
        Self { greeter }
    }

    #[process]
    pub async fn respond(&self, routes: &Routes, views: &Views) -> Response {
        Response::html(
            200,
            views.greeting_view.render(GreetingViewProps {
                greeting: self.greeter.greet(),
                routes,
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::GetGreetingCard;
    use crate::app_name::AppName;
    use crate::english_greeter::EnglishGreeter;
    use crate::margaret::routes::Routes;
    use crate::margaret::views::Views;
    use crate::views::card_layout::CardLayout;
    use crate::views::farewell_view::FarewellView;
    use crate::views::greeting_view::GreetingView;

    #[tokio::test]
    async fn responds_with_the_rendered_greeting_card() {
        let card_layout = Arc::new(CardLayout);
        let views = Views {
            card_layout: card_layout.clone(),
            farewell_view: Arc::new(FarewellView::create(card_layout.clone())),
            greeting_view: Arc::new(GreetingView::create(card_layout)),
        };
        let routes = Routes::from_origins(Arc::from("http://internal"), Arc::from("http://public"));
        let responder = GetGreetingCard::create(Arc::new(EnglishGreeter::create(Arc::new(
            AppName::create(),
        ))));

        assert_eq!(responder.respond(&routes, &views).await.status(), 200);
    }
}
