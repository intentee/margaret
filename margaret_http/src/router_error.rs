use matchit::InsertError;
use thiserror::Error;

use margaret_route_method::route_method::RouteMethod;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("the route path '{path}' answers the {method:?} method more than once")]
    DuplicateMethod {
        method: RouteMethod,
        path: &'static str,
    },

    #[error(transparent)]
    Path(#[from] InsertError),
}
