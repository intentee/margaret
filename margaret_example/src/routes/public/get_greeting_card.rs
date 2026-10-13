use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::singletons::english_greeter::EnglishGreeter;
use crate::views::greeting_view::GreetingViewProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/greeting-card", server = "public")]
pub struct GetGreetingCard {
    greeter: Arc<EnglishGreeter>,
}

impl GetGreetingCard {
    #[must_use]
    #[constructor]
    pub fn create(greeter: Arc<EnglishGreeter>) -> Self {
        Self { greeter }
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
