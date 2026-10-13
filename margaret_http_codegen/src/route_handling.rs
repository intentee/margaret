use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

#[derive(Clone, Copy)]
pub(crate) enum RouteHandling {
    Content(ContentMethod),
    Head(RouteMethod),
}
