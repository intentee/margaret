use crate::deferred_interception::DeferredInterception;
use crate::forward::Forward;
use crate::response::Response;

pub enum ResponseContinuation {
    Done(Response),
    Forward(Forward),
    Intercept(Box<dyn DeferredInterception>),
}

impl From<Response> for ResponseContinuation {
    fn from(response: Response) -> Self {
        Self::Done(response)
    }
}

impl From<Forward> for ResponseContinuation {
    fn from(forward: Forward) -> Self {
        Self::Forward(forward)
    }
}
