use std::convert::Infallible;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::cookie_changes::CookieChanges;
use crate::entered_responders::EnteredResponders;
use crate::forward::Forward;
use crate::forward_target::ForwardTarget;
use crate::forwarded_route::ForwardedRoute;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait ForwardScope: Sized + Send + Sync + 'static {
    type Targets: Send + Sync + 'static;

    async fn conclude(
        targets: &Self::Targets,
        entered: EnteredResponders,
        request: Request,
        outcome: ResponseContinuation<Self>,
    ) -> Result<Response, HandlerError>;
}

#[async_trait]
impl<TTarget: ForwardTarget> ForwardScope for TTarget {
    type Targets = <TTarget as ForwardTarget>::Targets;

    async fn conclude(
        targets: &Self::Targets,
        mut entered: EnteredResponders,
        mut request: Request,
        mut outcome: ResponseContinuation<Self>,
    ) -> Result<Response, HandlerError> {
        loop {
            match outcome {
                ResponseContinuation::Done(response) => return Ok(response),
                ResponseContinuation::Forward(Forward { cookies, target }) => {
                    let ForwardedRoute {
                        handler,
                        name,
                        path_params,
                    } = target.resolve(targets);

                    if !entered.enter(&handler) {
                        return Err(HandlerError::ForwardCycle { responder: name });
                    }

                    request = request.with_path_params(path_params);
                    outcome = CookieChanges { cookies }.precede(handler.handle(&request).await?);
                }
                ResponseContinuation::Redirect(redirect) => return Ok(redirect.into_response()),
            }
        }
    }
}

#[async_trait]
impl ForwardScope for Infallible {
    type Targets = ();

    async fn conclude(
        _targets: &Self::Targets,
        _entered: EnteredResponders,
        _request: Request,
        outcome: ResponseContinuation<Self>,
    ) -> Result<Response, HandlerError> {
        match outcome {
            ResponseContinuation::Done(response) => Ok(response),
            ResponseContinuation::Redirect(redirect) => Ok(redirect.into_response()),
        }
    }
}
