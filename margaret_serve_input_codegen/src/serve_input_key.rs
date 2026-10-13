use crate::preparation::Preparation;
use crate::route_url_input::RouteUrlInput;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ServeInputKey {
    ConsoleArgument { name: String },
    EnvironmentVariable { name: String },
    Prepared(Preparation),
    RouteUrl(RouteUrlInput),
    Routes,
    SpiffeHttpClient,
    SpiffeIssuerRequestClient,
}
