use std::future::Future;
use std::pin::Pin;

use crate::handler_error::HandlerError;
use crate::response_continuation::ResponseContinuation;

pub type HandlerFuture<'request> =
    Pin<Box<dyn Future<Output = Result<ResponseContinuation, HandlerError>> + Send + 'request>>;
