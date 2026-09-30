use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::views::renders_view::RendersView;

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
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(greeter: Arc<EnglishGreeter>) -> anyhow::Result<Self> {
        Ok(Self { greeter })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, routes: &Routes, views: &Views) -> anyhow::Result<Response> {
        Ok({
            Response::html(
                200,
                views.greeting_view.render(GreetingViewProps {
                    greeting: self.greeter.greet(),
                    routes,
                })?,
            )
        })
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
    use crate::views::consent_view::ConsentView;
    use crate::views::farewell_view::FarewellView;
    use crate::views::greeting_view::GreetingView;

    #[tokio::test]
    async fn responds_with_the_rendered_greeting_card() {
        let card_layout = Arc::new(CardLayout);
        let views = Views {
            card_layout: card_layout.clone(),
            consent_view: Arc::new(
                ConsentView::create(card_layout.clone()).expect("the consent view is constructed"),
            ),
            farewell_view: Arc::new(
                FarewellView::create(card_layout.clone())
                    .expect("the farewell view is constructed"),
            ),
            greeting_view: Arc::new(
                GreetingView::create(card_layout).expect("the greeting view is constructed"),
            ),
        };
        let routes = Routes::from_origins(
            Arc::from("http://identity"),
            Arc::from("http://internal"),
            Arc::from("http://public"),
        );
        let app_name = AppName::create().expect("the app name is constructed");
        let greeter =
            EnglishGreeter::create(Arc::new(app_name)).expect("the English greeter is constructed");
        let responder =
            GetGreetingCard::create(Arc::new(greeter)).expect("the responder is constructed");

        assert_eq!(
            responder
                .respond(&routes, &views)
                .expect("the responder succeeds")
                .status(),
            200
        );
    }
}
