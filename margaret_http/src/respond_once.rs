use crate::entered_responders::EnteredResponders;
use crate::forward_scope::ForwardScope;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;

pub(crate) async fn respond_once<TTarget: ForwardScope>(
    targets: &TTarget::Targets,
    request: Request,
    first: Box<dyn OneShotHandler<TTarget>>,
) -> Response {
    let outcome = first.handle(&request).await;

    respond_to_outcome(targets, EnteredResponders::default(), request, outcome).await
}
