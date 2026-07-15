use crate::method_handler::MethodHandler;

pub struct RouteEntry {
    pub handlers: Vec<MethodHandler>,
    pub path: &'static str,
}

impl RouteEntry {
    #[must_use]
    pub fn new(path: &'static str, handlers: Vec<MethodHandler>) -> Self {
        Self { handlers, path }
    }
}
