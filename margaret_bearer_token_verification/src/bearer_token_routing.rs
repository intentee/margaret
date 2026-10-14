use margaret_http::response::Response;

use crate::routed_bearer_token::RoutedBearerToken;

pub enum BearerTokenRouting<'request, 'trusted> {
    Refused(Response),
    Routed(RoutedBearerToken<'request, 'trusted>),
}
