use crate::route_url_input::RouteUrlInput;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ServeInputKey {
    ConsoleArgument { name: String },
    EnvironmentVariable { name: String },
    RouteUrl(RouteUrlInput),
    Routes,
    SpiffeHttpClient,
}
