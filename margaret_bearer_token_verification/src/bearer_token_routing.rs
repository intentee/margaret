use margaret_http::response_continuation::ResponseContinuation;

use crate::routed_bearer_token::RoutedBearerToken;

pub enum BearerTokenRouting<'request, 'trusted> {
    Refused(ResponseContinuation),
    Routed(RoutedBearerToken<'request, 'trusted>),
}
