use std::future::Future;
use std::pin::Pin;

use margaret_handler_error::handler_error::HandlerError;

use crate::response_continuation::ResponseContinuation;

pub type HandlerFuture<'request, TTarget> = Pin<
    Box<dyn Future<Output = Result<ResponseContinuation<TTarget>, HandlerError>> + Send + 'request>,
>;
