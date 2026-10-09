use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UrlSource {
    Declared(String),
    Route(RouteUrlInput),
}
