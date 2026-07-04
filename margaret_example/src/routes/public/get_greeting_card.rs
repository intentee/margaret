use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::greeter::Greeter;
use crate::views::greeting_view::GreetingView;
use crate::views::view::View;

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
    pub async fn respond(&self) -> Box<dyn View> {
        Box::new(GreetingView {
            greeting: self.greeter.greet(),
        })
    }
}
