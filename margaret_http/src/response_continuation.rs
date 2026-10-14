use crate::forward::Forward;
use crate::redirect::Redirect;
use crate::response::Response;

pub enum ResponseContinuation<TTarget> {
    Done(Response),
    Forward(Forward<TTarget>),
    Redirect(Redirect),
}

impl<TTarget> From<Forward<TTarget>> for ResponseContinuation<TTarget> {
    fn from(forward: Forward<TTarget>) -> Self {
        Self::Forward(forward)
    }
}

impl<TTarget> From<Redirect> for ResponseContinuation<TTarget> {
    fn from(redirect: Redirect) -> Self {
        Self::Redirect(redirect)
    }
}

impl<TTarget> From<Response> for ResponseContinuation<TTarget> {
    fn from(response: Response) -> Self {
        Self::Done(response)
    }
}
