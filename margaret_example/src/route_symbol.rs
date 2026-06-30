use margaret_http::http_route_symbol::HttpRouteSymbol;

pub enum RouteSymbol {
    GetGreeting,
}

impl HttpRouteSymbol for RouteSymbol {
    fn route_key(&self) -> &'static str {
        match self {
            Self::GetGreeting => "get_greeting",
        }
    }
}
