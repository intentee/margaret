use crate::forward::Forward;
use crate::redirect::Redirect;
use crate::response::Response;
use crate::upgrade_handler::UpgradeHandler;

pub enum ResponseContinuation {
    Done(Response),
    Forward(Forward),
    Redirect(Redirect),
    Upgrade(Box<dyn UpgradeHandler>),
}

impl From<Forward> for ResponseContinuation {
    fn from(forward: Forward) -> Self {
        Self::Forward(forward)
    }
}

impl From<Redirect> for ResponseContinuation {
    fn from(redirect: Redirect) -> Self {
        Self::Redirect(redirect)
    }
}

impl From<Response> for ResponseContinuation {
    fn from(response: Response) -> Self {
        Self::Done(response)
    }
}
