use margaret_http::http_interceptable::HttpInterceptable;

use crate::views::view::View;

pub struct GreetingView {
    pub greeting: String,
}

impl HttpInterceptable for GreetingView {}

impl View for GreetingView {
    fn body(&self) -> String {
        self.greeting.clone()
    }
}
