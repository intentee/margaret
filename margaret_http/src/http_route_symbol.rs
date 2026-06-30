pub trait HttpRouteSymbol: Send + Sync {
    fn route_key(&self) -> &'static str;
}
