use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::greeter::Greeter;
use crate::margaret::routes::Routes;
use crate::views::greeting_view::greeting_view;
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
    pub async fn respond(&self, routes: &Routes) -> Response {
        Response::html(
            200,
            greeting_view(GreetingViewProps {
                greeting: self.greeter.greet(),
                home_url: routes.public.get_greeting.url(),
            }),
        )
    }
}
