use margaret_http::http_interceptable::HttpInterceptable;

use crate::views::view::View;

pub struct FarewellView {
    pub name: String,
}

impl HttpInterceptable for FarewellView {}

impl View for FarewellView {
    fn body(&self) -> String {
        format!("goodbye, {}", self.name)
    }
}
